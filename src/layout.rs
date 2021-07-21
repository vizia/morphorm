
use crate::Node;
use crate::Cache;
use crate::Hierarchy;
use crate::{Units, LayoutType, PositionType};

use smallvec::SmallVec;


#[derive(Debug, Clone, Copy)]
enum Axis {
    Before,
    Size,
    After,
}

#[derive(Clone, Copy)]
pub struct ComputedData<'a, N: for<'b> Node<'b>> {

    node: &'a N,

    value: f32,
    min: f32,
    max: f32,
    axis: Axis,
}

/// Perform a layout calculation on the visual tree of nodes, the resulting positions and sizes are stored within the provided cache
pub fn layout<'a, C, H>(cache: &mut C, hierarchy: &'a H, store: &'a <<H as Hierarchy<'a>>::Item as Node>::Data)
where
    C: Cache<Item = <H as Hierarchy<'a>>::Item>,
    H: Hierarchy<'a>,
{
    // Step 1 - Determine fist and last parent-directed child of each node and cache it
    // This needs to be done at least once before the rest of layout and when the position_type of a node changes
    for parent in hierarchy.down_iter() {

        let mut found_first = false;
        let mut last_child = None;

        for node in hierarchy.child_iter(&parent) {

            cache.set_stack_first_child(&node, false);
            
            let position_type = node.position_type(store).unwrap_or_default();

            match position_type {
                PositionType::ParentDirected => {
                    if !found_first {
                        found_first = true;
                        cache.set_stack_first_child(&node, true);
                    }
                    last_child = Some(node);
                }

                PositionType::SelfDirected => {
                    cache.set_stack_first_child(&node, true);
                    cache.set_stack_last_child(&node, true);
                }
            }
        }

        if let Some(last_child) = last_child {
            cache.set_stack_last_child(last_child, true);
        }
    }

    // Step 2 - Iterate up the hierarchy
    // This step is required to determine the sum and max width/height of child nodes 
    // to determine the width/height of parent nodes when set to Auto
    for node in hierarchy.up_iter() {

        // Skip non-visible nodes
        // if !node.is_visible(&store) {
        //     continue;
        // }
        
        
        let parent = hierarchy.parent(&node);

        let (parent_width, parent_height) = if let Some(parent) = parent {
            (cache.width(parent), cache.height(parent))
        } else {
            (0.0, 0.0)
        };

        // Reset the sum and max for the parent
        if let Some(parent) = parent {
            cache.set_child_width_sum(parent, 0.0);
            cache.set_child_height_sum(parent, 0.0);
            cache.set_child_width_max(parent, 0.0);
            cache.set_child_height_max(parent, 0.0);
        }

        let parent_layout_type = parent.layout_type(store).unwrap_or_default();

        let child_left = parent.child_left(store).unwrap_or_default();
        let child_right = parent.child_right(store).unwrap_or_default();
        let child_top = parent.child_top(store).unwrap_or_default();
        let child_bottom = parent.child_bottom(store).unwrap_or_default();

        let row_between = parent.row_between(store).unwrap_or_default();
        let col_between = parent.col_between(store).unwrap_or_default();


        let layout_type = node.layout_type(store).unwrap_or_default();

        let mut left = node.left(store).unwrap_or_default();
        let mut right = node.right(store).unwrap_or_default();
        let mut top = node.top(store).unwrap_or_default();
        let mut bottom = node.bottom(store).unwrap_or_default();


        let min_left = node.min_left(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
        let max_left = node.max_left(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
        let min_right = node.min_right(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
        let max_right = node.max_right(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
        let min_top = node.min_top(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
        let max_top = node.max_top(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
        let min_bottom = node.min_bottom(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
        let max_bottom = node.max_bottom(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);

        let width = node.width(store).unwrap_or(Units::Stretch(1.0));
        let height = node.height(store).unwrap_or(Units::Stretch(1.0));

        // If Auto, then set the minimum width to be at least the width_sum/width_max/row_max of the children (depending on layout type)
        let min_width = node.min_width(store).unwrap_or_default().value_or(parent_width, 
            match layout_type {
                LayoutType::Column => cache.child_width_max(&node),
                LayoutType::Row => cache.child_width_sum(&node),
                LayoutType::Grid => cache.grid_row_max(&node),
            }
        );

        let max_width = node.max_width(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);

        // If Auto, then set the minimum height to be at least the height_sum/height_max/col_max of the children (depending on layout type)
        let min_height = node.min_height(store).unwrap_or_default().value_or( parent_height,
                match layout_type {
                    LayoutType::Column => cache.child_height_sum(&node),
                    LayoutType::Row => cache.child_height_max(&node),
                    LayoutType::Grid => cache.grid_col_max(&node),
                }
        );

        let max_height = node.max_height(store).unwrap_or_default().value_or(parent_height, std::f32::MAX);

        let border_left = node.border_left(store).unwrap_or_default().value_or(parent_width, 0.0);
        let border_right = node.border_right(store).unwrap_or_default().value_or(parent_width, 0.0);
        let border_top = node.border_top(store).unwrap_or_default().value_or(parent_width, 0.0);
        let border_bottom = node.border_bottom(store).unwrap_or_default().value_or(parent_width, 0.0);

        // If left/right/top/bottom are Auto then the parent child_left/child_right/child_top/child_bottom overrides them
        // The override is also dependent on position in stack (first, last, other) and layout type
        match parent_layout_type {
            LayoutType::Column => {
                if top == Units::Auto {
                    if cache.stack_first_child(&node) {
                        top = child_top;
                    } else {
                        top = row_between;
                    }
                }

                if bottom == Units::Auto {
                    if cache.stack_last_child(&node) {
                        bottom = child_bottom;
                    }
                }
                
                if left == Units::Auto {
                    left = child_left;
                }

                if right == Units::Auto {
                    right = child_right;
                }
            }

            LayoutType::Row => {
                if left == Units::Auto {
                    if cache.stack_first_child(&node) {
                        left = child_left;
                    } else {
                        left = col_between;
                    }
                }

                if right == Units::Auto {
                    if cache.stack_last_child(&node) {
                        right = child_right;
                    }
                }
                
                if top == Units::Auto {
                    top = child_top;
                }

                if bottom == Units::Auto {
                    bottom = child_bottom;
                }
            }

            // Should grids have parent overrides? (probably not)
            _=> {}
        }

        let mut new_left = 0.0;
        let mut new_width = 0.0;
        let mut new_right = 0.0;

        let mut new_top = 0.0;
        let mut new_height = 0.0;
        let mut new_bottom = 0.0;

        let mut horizontal_used_space = 0.0;
        let mut vertical_used_space = 0.0;

        match parent_layout_type {
            LayoutType::Column | LayoutType::Row => {
                match left {
                    Units::Pixels(val) => {
                        new_left = val.clamp(min_left, max_left);
                        horizontal_used_space += new_left;
                    }
        
                    _ => {}
                }
        
                match width {
                    Units::Pixels(val) => {
                        new_width = val.clamp(min_width, max_width);
                        horizontal_used_space += new_width;
                    }
        
                    Units::Auto => {
                        match layout_type {
                            LayoutType::Column => {
                                new_width = cache.child_width_max(&node);
                            }
        
                            LayoutType::Row => {
                                new_width = cache.child_width_sum(&node);
                            }
        
                            LayoutType::Grid => {
                                new_width = cache.grid_row_max(&node);
                            }
                        }

                        new_width += border_left + border_right;
        
                        horizontal_used_space += new_width;
                    }
        
                    _ => {}
                }
        
                match right {
                    Units::Pixels(val) => {
                        new_right = val.clamp(min_right, max_right);
                        horizontal_used_space += new_right;
                    }
        
                    _ => {}
                }
        
                match top {
                    Units::Pixels(val) => {
                        new_top = val.clamp(min_top, max_top);
                        vertical_used_space += new_top;
                    }
        
                    _ => {}
                }
        
                match height {
                    Units::Pixels(val) => {
                        new_height = val.clamp(min_height, max_height);
                        vertical_used_space += new_height;
                    }
        
                    Units::Auto => {
                        match layout_type {
                            LayoutType::Column => {
                                new_height = cache.child_height_sum(&node);
                            }
        
                            LayoutType::Row => {
                                new_height = cache.child_height_max(&node);
                            }
        
                            LayoutType::Grid => {
                                new_height = cache.grid_col_max(&node);
                            }
                        }

                        new_height += border_top + border_bottom;
        
                        vertical_used_space += new_height;
                    }
        
                    _ => {}
                }
        
                match bottom {
                    Units::Pixels(val) => {
                        new_bottom = val.clamp(min_bottom, max_bottom);
                        vertical_used_space += new_bottom;
                    }
        
                    _ => {}
                }

                let position_type = node.position_type(store).unwrap_or_default();

                cache.set_width(&node, new_width);
                cache.set_height(&node, new_height);
                cache.set_left(&node, new_left);
                cache.set_right(&node, new_right);
                cache.set_top(&node, new_top);
                cache.set_bottom(&node, new_bottom);
                
                if let Some(parent) = &parent {
                    if position_type == PositionType::ParentDirected {
                        cache.set_child_height_sum(
                            &parent,
                            cache.child_height_sum(&parent) + vertical_used_space,
                        );
            
                        cache.set_child_height_max(
                            &parent,
                            vertical_used_space.max(cache.child_height_max(&parent)),
                        );
            
                        cache.set_child_width_sum(
                            &parent,
                            cache.child_width_sum(&parent) + horizontal_used_space,
                        );
            
                        cache.set_child_width_max(
                            &parent,
                            horizontal_used_space.max(cache.child_width_max(&parent)),
                        );
                    }                    
                } else {
                    break;
                }
            }

            LayoutType::Grid => {
                // TODO
            }
        }
    }
    
    // Step 3 - Iterate down the hierarchy
    for parent in hierarchy.down_iter() {

        // if !parent.is_visible(store) {
        //     continue;
        // }

        

        let parent_layout_type = parent.layout_type(store).unwrap_or_default();
        let child_left = parent.child_left(store).unwrap_or_default();
        let child_right = parent.child_right(store).unwrap_or_default();
        let child_top = parent.child_top(store).unwrap_or_default();
        let child_bottom = parent.child_bottom(store).unwrap_or_default();

        let row_between = parent.row_between(store).unwrap_or_default();
        let col_between = parent.col_between(store).unwrap_or_default();

        let mut parent_width = cache.width(&parent);
        let mut parent_height = cache.height(&parent);


        let parent_border_left = parent.border_left(store).unwrap_or_default().value_or(parent_width, 0.0);
        let parent_border_right = parent.border_right(store).unwrap_or_default().value_or(parent_width, 0.0);
        let parent_border_top = parent.border_top(store).unwrap_or_default().value_or(parent_width, 0.0);
        let parent_border_bottom = parent.border_bottom(store).unwrap_or_default().value_or(parent_width, 0.0);

        parent_width -= parent_border_left + parent_border_right;
        parent_height -= parent_border_top + parent_border_bottom;

        let mut parent_horizontal_free_space = parent_width;
        let mut parent_vertical_free_space = parent_height;
        let mut parent_horizontal_stretch_sum = 0.0;
        let mut parent_vertical_stretch_sum = 0.0;

        match parent_layout_type {
            LayoutType::Row | LayoutType::Column => {
                let mut horizontal_axis = SmallVec::<[ComputedData<<H as Hierarchy>::Item>; 3]>::new();
                let mut vertical_axis = SmallVec::<[ComputedData<<H as Hierarchy>::Item>; 3]>::new();

                
                // ////////////////////////////////
                // Calculate inflexible children //
                ///////////////////////////////////
                for node in hierarchy.child_iter(&parent) {

                    // if !parent.is_visible(store) {
                    //     continue;
                    // }

                    let layout_type = node.layout_type(store).unwrap_or_default();

                    let mut left = node.left(store).unwrap_or_default();
                    let mut right = node.right(store).unwrap_or_default();
                    let mut top = node.top(store).unwrap_or_default();
                    let mut bottom = node.bottom(store).unwrap_or_default();
            
            
                    let min_left = node.min_left(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
                    let max_left = node.max_left(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
                    let min_right = node.min_right(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
                    let max_right = node.max_right(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
                    let min_top = node.min_top(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
                    let max_top = node.max_top(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
                    let min_bottom = node.min_bottom(store).unwrap_or_default().value_or(parent_width, -std::f32::MAX);
                    let max_bottom = node.max_bottom(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
            
                    let width = node.width(store).unwrap_or(Units::Stretch(1.0));
                    let height = node.height(store).unwrap_or(Units::Stretch(1.0));
            
                    // This could be cached during up phase because it shouldn't change between up phase and down phase
                    let min_width = node.min_width(store).unwrap_or_default().value_or(parent_width, 
                        match layout_type {
                            LayoutType::Column => cache.child_width_max(&node),
                            LayoutType::Row => cache.child_width_sum(&node),
                            LayoutType::Grid => cache.grid_row_max(&node),
                        }
                    );
            
                    let max_width = node.max_width(store).unwrap_or_default().value_or(parent_width, std::f32::MAX);
            
                    // This could be cached during up phase because it shouldn't change between up phase and down phase
                    let min_height = node.min_height(store).unwrap_or_default().value_or( parent_height,
                            match layout_type {
                                LayoutType::Column => cache.child_height_sum(&node),
                                LayoutType::Row => cache.child_height_max(&node),
                                LayoutType::Grid => cache.grid_col_max(&node),
                            }
                    );
            
                    let max_height = node.max_height(store).unwrap_or_default().value_or(parent_height, std::f32::MAX);
            
                    let border_left = node.border_left(store).unwrap_or_default().value_or(parent_width, 0.0);
                    let border_right = node.border_right(store).unwrap_or_default().value_or(parent_width, 0.0);
                    let border_top = node.border_top(store).unwrap_or_default().value_or(parent_width, 0.0);
                    let border_bottom = node.border_bottom(store).unwrap_or_default().value_or(parent_width, 0.0);
            
                    let position_type = node.position_type(store).unwrap_or_default();

                    // Parent overrides
                    match parent_layout_type {
                        LayoutType::Column => {
                            if top == Units::Auto {
                                if cache.stack_first_child(&node) {
                                    top = child_top;
                                } else {
                                    top = row_between;
                                }
                            }
            
                            if bottom == Units::Auto {
                                if cache.stack_last_child(&node) {
                                    bottom = child_bottom;
                                }
                            }
                            
                            if left == Units::Auto {
                                left = child_left;
                            }
            
                            if right == Units::Auto {
                                right = child_right;
                            }
                        }
            
                        LayoutType::Row => {
                            if left == Units::Auto {
                                if cache.stack_first_child(&node) {
                                    left = child_left;
                                } else {
                                    left = col_between;
                                }
                            }
            
                            if right == Units::Auto {
                                if cache.stack_last_child(&node) {
                                    right = child_right;
                                }
                            }
                            
                            if top == Units::Auto {
                                top = child_top;
                            }
            
                            if bottom == Units::Auto {
                                bottom = child_bottom;
                            }
                        }
            
                        _=> {}
                    }

                    let mut new_left = 0.0;
                    let mut new_width = 0.0;
                    let mut new_right = 0.0;

                    let mut new_top = 0.0;
                    let mut new_height = 0.0;
                    let mut new_bottom = 0.0;

                    let mut horizontal_stretch_sum = 0.0;
                    let mut vertical_stretch_sum = 0.0;

                    let mut horizontal_free_space = parent_width;
                    let mut vertical_free_space = parent_height;

                    
                    

                    // TODO - replace all these match' with a function
                    match left {
                        Units::Pixels(val) => {
                            new_left = val.clamp(min_left, max_left);
                            horizontal_free_space -= new_left;
                        }

                        Units::Percentage(val) => {
                            new_left = (val * parent_width).round();
                            new_left = new_left.clamp(min_left, max_left);
                            horizontal_free_space -= new_left;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push(ComputedData {
                                node,
                                value: val,
                                min: min_left,
                                max: max_left,
                                axis: Axis::Before,
                            });                        
                        }

                        _ => {}
                    }

                    match width {
                        Units::Pixels(val) => {
                            new_width = val.clamp(min_width, max_width);
                            horizontal_free_space -= new_width;
                        }

                        Units::Percentage(val) => {
                            new_width = (val/100.0) * parent_width;
                            new_width = new_width.clamp(min_width, max_width);
                            horizontal_free_space -= new_width;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push(
                                ComputedData {
                                    node,
                                    value: val,
                                    min: min_width,
                                    max: max_width,
                                    axis: Axis::Size,
                                }
                            );
                        }

                        Units::Auto => {
                            match layout_type {
                                LayoutType::Column => {
                                    new_width =
                                        cache.child_width_max(&node);
                                }

                                LayoutType::Row | LayoutType::Grid=> {
                                    new_width =
                                        cache.child_width_sum(&node);
                                }
                            }

                            new_width += border_left + border_right;
                            horizontal_free_space -= new_width;
                        }
                    }

                    match right {
                        Units::Pixels(val) => {
                            new_right = val.clamp(min_right, max_right);
                            horizontal_free_space -= new_right;
                        }

                        Units::Percentage(val) => {
                            new_right = (val * parent_width).round();
                            new_right = new_right.clamp(min_right, max_right);
                            horizontal_free_space -= new_right;
                        }

                        Units::Stretch(val) => {
                            horizontal_stretch_sum += val;
                            horizontal_axis.push(
                                ComputedData {
                                    node,
                                    value: val,
                                    min: min_right,
                                    max: max_right,
                                    axis: Axis::After,
                                }
                            );
                        }

                        _ => {}
                    }

                    match top {
                        Units::Pixels(val) => {
                            new_top = val.clamp(min_top, max_top);
                            vertical_free_space -= new_top;
                        }

                        Units::Percentage(val) => {
                            new_top = (val * parent_height).round();
                            new_top = new_top.clamp(min_top, max_top);
                            vertical_free_space -= new_top;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push(
                                ComputedData {
                                    node,
                                    value: val,
                                    min: min_bottom,
                                    max: max_bottom,
                                    axis: Axis::Before,
                                }
                            );
                        }

                        _ => {}
                    }

                    match height {
                        Units::Pixels(val) => {
                            new_height = val.clamp(min_height, max_height);
                            vertical_free_space -= new_height;
                        }

                        Units::Percentage(val) => {
                            new_height = (val * parent_height).round();
                            new_height = new_height.clamp(min_height, max_height);
                            vertical_free_space -= new_height;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push(
                                ComputedData {
                                    node,
                                    value: val,
                                    min: min_bottom,
                                    max: max_bottom,
                                    axis: Axis::Size,
                                }
                            );
                        }

                        Units::Auto => {
                            match layout_type {
                                LayoutType::Column | LayoutType::Grid => {
                                    new_height =
                                        cache.child_height_sum(&node);
                                }

                                LayoutType::Row => {
                                    new_height =
                                        cache.child_height_max(&node);
                                }
                            }

                            new_height += border_top + border_bottom;
                            vertical_free_space -= new_height;
                        }
                    }

                    match bottom {
                        Units::Pixels(val) => {
                            new_bottom = val.clamp(min_bottom, max_bottom);
                            vertical_free_space -= val;
                        }

                        Units::Percentage(val) => {
                            new_bottom = (val * parent_height).round();
                            new_bottom = new_bottom.clamp(min_bottom, max_bottom);
                            vertical_free_space -= new_bottom;
                        }

                        Units::Stretch(val) => {
                            vertical_stretch_sum += val;
                            vertical_axis.push(
                                ComputedData {
                                    node,
                                    value: val,
                                    min: min_bottom,
                                    max: max_bottom,
                                    axis: Axis::After,
                                }
                            );
                        }

                        _ => {}
                    }

                    cache.set_width(&node, new_width);
                    cache.set_height(&node, new_height);
                    cache.set_left(&node, new_left);
                    cache.set_right(&node, new_right);
                    cache.set_top(&node, new_top);
                    cache.set_bottom(&node, new_bottom);
                
                    if position_type == PositionType::ParentDirected {
                        parent_vertical_free_space -= parent_height - vertical_free_space;
                        parent_horizontal_free_space -= parent_width - horizontal_free_space;
                        parent_vertical_stretch_sum += vertical_stretch_sum;
                        parent_horizontal_stretch_sum += horizontal_stretch_sum;
                    }

                    cache
                        .set_horizontal_free_space(&node, horizontal_free_space);
                    cache
                        .set_horizontal_stretch_sum(&node, horizontal_stretch_sum);
                    cache
                        .set_vertical_free_space(&node, vertical_free_space);
                    cache
                        .set_vertical_stretch_sum(&node, vertical_stretch_sum);
                

                    
                
                }



                if parent_horizontal_stretch_sum == 0.0 {
                    parent_horizontal_stretch_sum = 1.0;
                }

                if parent_vertical_stretch_sum == 0.0 {
                    parent_vertical_stretch_sum = 1.0;
                }

                // Sort the stretch elements in each axis by the maximum size
                horizontal_axis.sort_by(|a, b| a.max.partial_cmp(&b.max).unwrap());
                vertical_axis.sort_by(|a, b| a.max.partial_cmp(&b.max).unwrap());

                let mut horizontal_stretch_sum = 0.0;
                let mut horizontal_free_space = 0.0;
                let mut vertical_stretch_sum = 0.0;
                let mut vertical_free_space = 0.0;

                /////////////////////////////////////////
                // Calculate flexible Row space & size //
                /////////////////////////////////////////
                for computed_data in horizontal_axis.iter() {
                    
                    let node = computed_data.node.clone();

                    let position_type = node.position_type(store).unwrap_or_default();

                    match position_type {
                        PositionType::SelfDirected => {
                            horizontal_free_space = cache.horizontal_free_space(&node);
                            horizontal_stretch_sum = cache.horizontal_stretch_sum(&node);
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Row => {
                                    horizontal_stretch_sum = parent_horizontal_stretch_sum;
                                    horizontal_free_space = parent_horizontal_free_space;
                                }

                                LayoutType::Column => {
                                    horizontal_free_space = cache.horizontal_free_space(&node);
                                    horizontal_stretch_sum = cache.horizontal_stretch_sum(&node);
                                }

                                _=> {}
                            }
                            
                        }
                    }

                    // Prevent a divide by zero when the stretch sum is 0
                    if horizontal_stretch_sum == 0.0 {
                        horizontal_stretch_sum = 1.0;
                    }

                    // Compute the new left/width/height based on free space, stretch factor, and stretch_sum
                    let mut new_value = horizontal_free_space * computed_data.value / horizontal_stretch_sum;

                    // Clamp the new left/width/right to be between min_ left/width/right and max_ left/width/right
                    new_value = new_value.clamp(computed_data.min, computed_data.max);

                    // Could perhaps replace this with a closure
                    match computed_data.axis {
                        Axis::Before => {
                            cache.set_left(&node, new_value);
                        }

                        Axis::Size => {
                            cache.set_width(&node, new_value);
                        }

                        Axis::After => {
                            cache.set_right(&node, new_value);
                        }
                    }

                    match position_type {
                        PositionType::SelfDirected => {
                            cache.set_horizontal_stretch_sum(&node, horizontal_stretch_sum - computed_data.value);
                            cache.set_horizontal_free_space(
                                &node,
                                horizontal_free_space - new_value,
                            );
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Column => {
                                    cache.set_horizontal_stretch_sum(
                                        &node,
                                        horizontal_stretch_sum - computed_data.value,
                                    );
                                    cache.set_horizontal_free_space(
                                        &node,
                                        horizontal_free_space - new_value,
                                    );
                                }

                                LayoutType::Row => {
                                    horizontal_free_space -= new_value;
                                    horizontal_stretch_sum -= computed_data.value;
                                }

                                _ => {}
                            };
                        }
                    }
                }

                ////////////////////////////////////////////
                // Calculate flexible Column space & size //
                ////////////////////////////////////////////
                for computed_data in vertical_axis.iter() {

                    let node = computed_data.node.clone();

                    let position_type = node.position_type(store).unwrap_or_default();

                    match position_type {
                        PositionType::SelfDirected => {
                            vertical_free_space = cache.vertical_free_space(&node);
                            vertical_stretch_sum = cache.vertical_stretch_sum(&node);
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Column => {
                                    vertical_stretch_sum = parent_vertical_stretch_sum;
                                    vertical_free_space = parent_vertical_free_space;
                                }

                                LayoutType::Row => {
                                    vertical_free_space = cache.vertical_free_space(&node);
                                    vertical_stretch_sum = cache.vertical_stretch_sum(&node);
                                }     
                                
                                _=> {}
                            }

                        }
                    }

                    if vertical_stretch_sum == 0.0 {
                        vertical_stretch_sum = 1.0;
                    }

                    
                    let mut new_value = vertical_free_space * computed_data.value / vertical_stretch_sum;
                    new_value = new_value.clamp(computed_data.min, computed_data.max);

                    match computed_data.axis {
                        Axis::Before => {
                            cache.set_top(&node, new_value);
                        }

                        Axis::Size => {
                            cache.set_height(&node, new_value);
                        }

                        Axis::After => {
                            cache.set_bottom(&node, new_value);
                        }
                    }

                    match position_type {
                        PositionType::SelfDirected => {
                            cache.set_vertical_stretch_sum(&node, vertical_stretch_sum - computed_data.value);
                            cache.set_vertical_free_space(
                                &node,
                                vertical_free_space - new_value,
                            );
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Row => {
                                    cache.set_vertical_stretch_sum(
                                        &node,
                                        vertical_stretch_sum - computed_data.value,
                                    );
                                    cache.set_vertical_free_space(
                                        &node,
                                        vertical_free_space - new_value,
                                    );
                                }

                                LayoutType::Column => {
                                    parent_vertical_free_space -= new_value;
                                    parent_vertical_stretch_sum -= computed_data.value;
                                }

                                _ => {}
                            };
                        }
                    }
                }

                let mut current_posx = 0.0;
                let mut current_posy = 0.0;

                let parent_posx = cache.posx(&parent) + parent_border_left;
                let parent_posy = cache.posy(&parent) + parent_border_top;

                ///////////////////////
                // Position Children //
                ///////////////////////
                for node in hierarchy.child_iter(&parent) {

                    // if !node.is_visible(store) {
                    //     continue;
                    // }

                    let left = cache.left(&node);
                    let right = cache.right(&node);
                    let top = cache.top(&node);
                    let bottom = cache.bottom(&node);

                    let width = cache.width(&node);
                    let height = cache.height(&node);

                    let position_type = node.position_type(store).unwrap_or_default();

                    let (new_posx, new_posy) = match position_type {
                        PositionType::SelfDirected => {
                            (parent_posx + left, parent_posy + top)
                        }

                        PositionType::ParentDirected => {
                            let new_posx = parent_posx + current_posx + left;
                            let new_posy = parent_posy + current_posy + top;

                            match parent_layout_type {
                                LayoutType::Column => {
                                    current_posy += top + height + bottom;
                                }

                                LayoutType::Row => {
                                    current_posx += left + width + right;
                                }

                                _ => {}
                            }

                            (new_posx, new_posy)
                        }
                    };

                    cache.set_posx(&node, new_posx);
                    cache.set_posy(&node, new_posy);
                }
            }

            LayoutType::Grid => {
                /////////////////////////////////////////////////////
                // Determine Size of non-flexible rows and columns //
                /////////////////////////////////////////////////////
                let grid_rows = parent.grid_rows(store).unwrap_or_default();
                let grid_cols = parent.grid_cols(store).unwrap_or_default();

                let mut row_heights = vec![(0.0, 0.0); grid_rows.len() + 1];
                let mut col_widths = vec![(0.0, 0.0,); grid_cols.len() + 1];

                let mut col_free_space = parent_width;
                let mut row_free_space = parent_height;

                let mut row_stretch_sum = 0.0;
                let mut col_stretch_sum = 0.0;

                let row_between = parent.row_between(store).unwrap_or_default().value_or(parent_height, 0.0);
                let col_between = parent.col_between(store).unwrap_or_default().value_or(parent_width, 0.0);

                for (i, row) in grid_rows.iter().enumerate() {
                    match row {
                        &Units::Pixels(val) => {
                            row_heights[i].1 = val;
                            row_free_space -= val;
                        }

                        &Units::Stretch(val) => {
                            row_stretch_sum += val;
                        }

                        _ => {}
                    }
                }

                for (i, col) in grid_cols.iter().enumerate() {
                    match col {
                        &Units::Pixels(val) => {
                            col_widths[i].1 = val;
                            col_free_space -= val;
                        }

                        &Units::Stretch(val) => {
                            col_stretch_sum += val;
                        }

                        _ => {}
                    }
                }


                if row_stretch_sum == 0.0 {
                    row_stretch_sum = 1.0;
                }
                if col_stretch_sum == 0.0 {
                    col_stretch_sum = 1.0;
                }

                /////////////////////////////////////////////////
                // Determine Size of flexible rows and columns //
                /////////////////////////////////////////////////
                let mut current_row_pos = cache.posy(&parent);
                let mut current_col_pos = cache.posx(&parent);

                for (i, row) in grid_rows.iter().enumerate() {
                    match row {
                        &Units::Stretch(val) => {
                            row_heights[i].1 = row_free_space * val / row_stretch_sum;
                        }

                        _ => {}
                    }

                    row_heights[i].0 = current_row_pos;
                    current_row_pos += row_heights[i].1;
                }
                let row_heights_len = row_heights.len() - 1;
                row_heights[row_heights_len].0 = current_row_pos;

                for (i, col) in grid_cols.iter().enumerate() {
                    match col {
                        &Units::Stretch(val) => {
                            col_widths[i].1 = col_free_space * val / col_stretch_sum;
                        }

                        _ => {}
                    }

                    col_widths[i].0 = current_col_pos;

                    current_col_pos += col_widths[i].1;
                }

                let col_widths_len = col_widths.len() - 1;
                col_widths[col_widths_len].0 = current_col_pos;


                ///////////////////////////////////////////////////
                // Position and Size child nodes within the grid //
                ///////////////////////////////////////////////////
                for node in hierarchy.child_iter(&parent) {

                    // if !node.is_visible(store) {
                    //     continue;
                    // }

                    let row_start = node.row_index(store).unwrap_or_default();
                    let row_end = row_start + node.row_span(store).unwrap_or_default();

                    let col_start = node.col_index(store).unwrap_or_default();
                    let col_end = col_start + node.col_span(store).unwrap_or_default();

                    // Set posx and width based on col_index and col_start
                    if col_start == 0 {
                        cache.set_posx(&node, col_widths[col_start].0);
                        cache.set_width(
                            &node,
                            (col_widths[col_end].0 - col_widths[col_start].0)
                                - col_between / 2.0,
                        );
                    } else if col_end + 1 == col_widths.len() {
                        cache
                            .set_posx(&node, col_widths[col_start].0 + (col_between / 2.0));
                        cache.set_width(
                            &node,
                            (col_widths[col_end].0 - col_widths[col_start].0)
                                - col_between / 2.0,
                        );
                    } else {
                        cache
                            .set_posx(&node, col_widths[col_start].0 + (col_between / 2.0));
                        cache.set_width(
                            &node,
                            (col_widths[col_end].0 - col_widths[col_start].0) - col_between,
                        );
                    }

                    // Set posy and height based on row_index and row_span
                    if row_start == 0 {
                        cache.set_posy(&node, row_heights[row_start].0);
                        cache.set_height(
                            &node,
                            (row_heights[row_end].0 - row_heights[row_start].0)
                                - row_between / 2.0,
                        );
                    } else if row_end + 1 == row_heights.len() {
                        cache
                            .set_posy(&node, row_heights[row_start].0 + (row_between / 2.0));
                        cache.set_height(
                            &node,
                            (row_heights[row_end].0 - row_heights[row_start].0)
                                - row_between / 2.0,
                        );
                    } else {
                        cache
                            .set_posy(&node, row_heights[row_start].0 + (row_between / 2.0));
                        cache.set_height(
                            &node,
                            (row_heights[row_end].0 - row_heights[row_start].0) - row_between,
                        );
                    }
                }
            }
        }
    }
}