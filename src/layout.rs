
use crate::Node;
use crate::Cache;
use crate::Hierarchy;
use crate::{Units, LayoutType, PositionType, GeometryChanged};

use smallvec::SmallVec;


#[derive(Debug, Clone, Copy, PartialEq)]
enum Axis {
    Before,
    Size,
    After,
}

#[derive(Clone, Copy)]
pub struct ComputedData<N: for<'b> Node<'b>> {

    node: N,

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

        // Skip non-visible nodes
        let visible = cache.visible(parent);
        if !visible {
            continue;
        }

        // Reset the sum and max for the parent
        cache.set_child_main_sum(parent, 0.0);
        cache.set_child_cross_sum(parent, 0.0);
        cache.set_child_main_max(parent, 0.0);
        cache.set_child_cross_max(parent, 0.0);
        

        cache.set_geo_changed(parent, GeometryChanged::POSX_CHANGED, false);
        cache.set_geo_changed(parent, GeometryChanged::POSY_CHANGED, false);
        cache.set_geo_changed(parent, GeometryChanged::WIDTH_CHANGED, false);
        cache.set_geo_changed(parent, GeometryChanged::HEIGHT_CHANGED, false);
        
        

        let mut found_first = false;
        let mut last_child = None;

        for node in hierarchy.child_iter(parent) {

            // Skip non-visible nodes
            let visible = cache.visible(node);
            if !visible {
                continue;
            }

            cache.set_stack_first_child(node, false);
            cache.set_stack_last_child(node, false);
            
            let position_type = node.position_type(store).unwrap_or_default();

            match position_type {
                PositionType::ParentDirected => {
                    if !found_first {
                        found_first = true;
                        cache.set_stack_first_child(node, true);
                    }
                    last_child = Some(node);
                }

                PositionType::SelfDirected => {
                    cache.set_stack_first_child(node, true);
                    cache.set_stack_last_child(node, true);
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
        let visible = cache.visible(node);
        if !visible {
            continue;
        }
        
        let parent = hierarchy.parent(node);

        let (parent_main, parent_cross) = if let Some(parent) = parent {
            (cache.new_main(parent), cache.new_cross(parent))
        } else {
            (0.0, 0.0)
        };

        let parent_layout_type = parent.map_or(None, |parent| parent.layout_type(store)).unwrap_or_default();

        let child_main_before = parent.map_or(None, |parent| parent.child_main_before(store)).unwrap_or_default();
        let child_main_after = parent.map_or(None, |parent| parent.child_main_after(store)).unwrap_or_default();
        let child_cross_before = parent.map_or(None, |parent| parent.child_cross_before(store)).unwrap_or_default();
        let child_cross_after = parent.map_or(None, |parent| parent.child_cross_after(store)).unwrap_or_default();

        let main_between = parent.map_or(None, |parent| parent.main_between(store)).unwrap_or_default();
        let cross_between = parent.map_or(None, |parent| parent.cross_between(store)).unwrap_or_default();


        let layout_type = node.layout_type(store).unwrap_or_default();

        let mut main_before = node.main_before(store).unwrap_or_default();
        let mut main_after = node.main_after(store).unwrap_or_default();
        let mut cross_before = node.cross_before(store).unwrap_or_default();
        let mut cross_after = node.cross_after(store).unwrap_or_default();


        let min_main_before = node.min_main_before(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
        let max_main_before = node.max_main_before(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
        let min_main_after = node.min_main_after(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
        let max_main_after = node.max_main_after(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
        let min_cross_before = node.min_cross_before(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
        let max_cross_before = node.max_cross_before(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
        let min_cross_after = node.min_cross_after(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
        let max_cross_after = node.max_cross_after(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);

        let main = node.main(store).unwrap_or(Units::Stretch(1.0));
        let cross = node.cross(store).unwrap_or(Units::Stretch(1.0));

        // If Auto, then set the minimum width to be at least the width_sum/width_max/row_max of the children (depending on layout type)
        let min_main = node.min_main(store).unwrap_or_default().value_or(parent_main, 
            match layout_type {
                LayoutType::Column => cache.child_main_max(node),
                LayoutType::Row => cache.child_main_sum(node),
                LayoutType::Grid => cache.grid_row_max(node),
            }
        );

        let max_main = node.max_main(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);

        // If Auto, then set the minimum height to be at least the height_sum/height_max/col_max of the children (depending on layout type)
        let min_cross = node.min_cross(store).unwrap_or_default().value_or( parent_cross,
                match layout_type {
                    LayoutType::Column => cache.child_cross_sum(node),
                    LayoutType::Row => cache.child_cross_max(node),
                    LayoutType::Grid => cache.grid_col_max(node),
                }
        );

        let max_cross = node.max_cross(store).unwrap_or_default().value_or(parent_cross, std::f32::MAX);

        let border_left = node.border_left(store).unwrap_or_default().value_or(parent_main, 0.0);
        let border_right = node.border_right(store).unwrap_or_default().value_or(parent_main, 0.0);
        let border_top = node.border_top(store).unwrap_or_default().value_or(parent_main, 0.0);
        let border_bottom = node.border_bottom(store).unwrap_or_default().value_or(parent_main, 0.0);

        // If left/right/top/bottom are Auto then the parent child_left/child_right/child_top/child_bottom overrides them
        // The override is also dependent on position in stack (first, last, other) and layout type
        match parent_layout_type {
            LayoutType::Column | LayoutType::Row => {
                if main_before == Units::Auto {
                    if cache.stack_first_child(node) {
                        main_before = child_main_before;
                    } else {
                        main_before = main_between;
                    }
                }

                if main_after == Units::Auto {
                    if cache.stack_last_child(node) {
                        main_after = child_main_after;
                    }
                }
                
                if cross_before == Units::Auto {
                    cross_before = child_cross_before;
                }

                if cross_after == Units::Auto {
                    cross_after = child_cross_after;
                }
            }

            // Should grids have parent overrides? (probably not)
            _=> {}
        }

        let mut new_main_before = 0.0;
        let mut new_main = 0.0;
        let mut new_main_after = 0.0;

        let mut new_cross_before = 0.0;
        let mut new_cross = 0.0;
        let mut new_cross_after = 0.0;

        let mut main_used_space = 0.0;
        let mut cross_used_space = 0.0;

        match parent_layout_type {
            LayoutType::Column | LayoutType::Row => {
                match main_before {
                    Units::Pixels(val) => {
                        new_main_before = val.clamp(min_main_before, max_main_before);
                        //horizontal_used_space += new_left;
                    }
        
                    _ => {}
                }
        
                match main {
                    Units::Pixels(val) => {
                        new_main = val.clamp(min_main, max_main);
                        //horizontal_used_space += new_width;
                    }
        
                    Units::Auto => {
                        match layout_type {
                            LayoutType::Column => {
                                new_main = cache.child_main_max(node);
                            }
        
                            LayoutType::Row => {
                                new_main = cache.child_main_sum(node);
                            }
        
                            LayoutType::Grid => {
                                new_main = cache.grid_row_max(node);
                            }
                        }

                        new_main = new_main.clamp(min_main, max_main);

                        new_main += border_left + border_right;
        
                        //horizontal_used_space += new_width.max(min_width);
                        //println!("{:?} hus: {}", node, horizontal_used_space);
                    }
        
                    _ => {}
                }

                main_used_space += new_main.max(min_main);
        
                match main_after {
                    Units::Pixels(val) => {
                        new_main_after = val.clamp(min_main_after, max_main_after);
                        main_used_space += new_main_after;
                    }
        
                    _ => {}
                }
        
                match cross_before {
                    Units::Pixels(val) => {
                        new_cross_before = val.clamp(min_cross_before, max_cross_before);
                        cross_used_space += new_cross_before;
                    }
        
                    _ => {}
                }
        
                match cross {
                    Units::Pixels(val) => {
                        new_cross = val.clamp(min_cross, max_cross);
                        cross_used_space += new_cross;
                    }
        
                    Units::Auto => {
                        match layout_type {
                            LayoutType::Column => {
                                new_cross = cache.child_cross_sum(node);
                            }
        
                            LayoutType::Row => {
                                new_cross = cache.child_cross_max(node);
                            }
        
                            LayoutType::Grid => {
                                new_cross = cache.grid_col_max(node);
                            }
                        }

                        new_cross = new_cross.clamp(min_cross, max_cross);

                        new_cross += border_top + border_bottom;
        
                        cross_used_space += new_cross;
                    }
        
                    _ => {}
                }
        
                match cross_after {
                    Units::Pixels(val) => {
                        new_cross_after = val.clamp(min_cross_after, max_cross_after);
                        cross_used_space += new_cross_after;
                    }
        
                    _ => {}
                }

                let position_type = node.position_type(store).unwrap_or_default();

                cache.set_new_main(node, new_main);
                cache.set_new_cross(node, new_cross);
                cache.set_main_before(node, new_main_before);
                cache.set_main_after(node, new_main_after);
                cache.set_cross_before(node, new_cross_before);
                cache.set_cross_after(node, new_cross_after);
                
                if let Some(parent) = parent {
                    if position_type == PositionType::ParentDirected {

                        cache.set_child_main_sum(
                            parent,
                            cache.child_main_sum(parent) + main_used_space,
                        );

                        cache.set_child_main_max(
                            parent,
                            main_used_space.max(cache.child_main_max(parent)),
                        );
            
                        cache.set_child_cross_sum(
                            parent,
                            cache.child_cross_sum(parent) + cross_used_space,
                        );
            
                        cache.set_child_cross_max(
                            parent,
                            cross_used_space.max(cache.child_cross_max(parent)),
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

        let visible = cache.visible(parent);
        if !visible {
            continue;
        }

        

        let parent_layout_type = parent.layout_type(store).unwrap_or_default();
        let child_main_before = parent.child_main_before(store).unwrap_or_default();
        let child_main_after = parent.child_main_after(store).unwrap_or_default();
        let child_cross_before = parent.child_cross_before(store).unwrap_or_default();
        let child_cross_after = parent.child_cross_after(store).unwrap_or_default();

        let main_between = parent.main_between(store).unwrap_or_default();
        let cross_between = parent.cross_between(store).unwrap_or_default();

        let mut parent_main = cache.new_main(parent);
        let mut parent_cross = cache.new_cross(parent);


        let parent_border_left = parent.border_left(store).unwrap_or_default().value_or(parent_main, 0.0);
        let parent_border_right = parent.border_right(store).unwrap_or_default().value_or(parent_main, 0.0);
        let parent_border_top = parent.border_top(store).unwrap_or_default().value_or(parent_main, 0.0);
        let parent_border_bottom = parent.border_bottom(store).unwrap_or_default().value_or(parent_main, 0.0);

        parent_main -= parent_border_left + parent_border_right;
        parent_cross -= parent_border_top + parent_border_bottom;

        let mut parent_main_free_space = parent_main;
        let mut parent_cross_free_space = parent_cross;
        let mut parent_main_stretch_sum = 0.0;
        let mut parent_cross_stretch_sum = 0.0;

        match parent_layout_type {
            LayoutType::Row | LayoutType::Column => {
                let mut main_axis = SmallVec::<[ComputedData<<H as Hierarchy>::Item>; 3]>::new();
                let mut cross_axis = SmallVec::<[ComputedData<<H as Hierarchy>::Item>; 3]>::new();

                
                // ////////////////////////////////
                // Calculate inflexible children //
                ///////////////////////////////////
                for node in hierarchy.child_iter(parent) {

                    let visible = cache.visible(node);
                    if !visible {
                        continue;
                    }

                    let layout_type = node.layout_type(store).unwrap_or_default();

                    let mut main_before = node.main_before(store).unwrap_or_default();
                    let mut main_after = node.main_after(store).unwrap_or_default();
                    let mut cross_before = node.cross_before(store).unwrap_or_default();
                    let mut cross_after = node.cross_after(store).unwrap_or_default();
            
            
                    let min_main_before = node.min_main_before(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
                    let max_main_before = node.max_main_before(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
                    let min_main_after = node.min_main_after(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
                    let max_main_after = node.max_main_after(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
                    let min_cross_before = node.min_cross_before(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
                    let max_cross_before = node.max_cross_before(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
                    let min_cross_after = node.min_cross_after(store).unwrap_or_default().value_or(parent_main, -std::f32::MAX);
                    let max_cross_after = node.max_cross_after(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
            
                    let main = node.main(store).unwrap_or(Units::Stretch(1.0));
                    let cross = node.cross(store).unwrap_or(Units::Stretch(1.0));
            
                    // This could be cached during up phase because it shouldn't change between up phase and down phase
                    let min_main = node.min_main(store).unwrap_or_default().value_or(parent_main, 
                        match layout_type {
                            LayoutType::Column => cache.child_main_max(node),
                            LayoutType::Row => cache.child_main_sum(node),
                            LayoutType::Grid => cache.grid_row_max(node),
                        }
                    );
            
                    let max_main = node.max_main(store).unwrap_or_default().value_or(parent_main, std::f32::MAX);
            
                    // This could be cached during up phase because it shouldn't change between up phase and down phase
                    let min_cross = node.min_cross(store).unwrap_or_default().value_or( parent_cross,
                            match layout_type {
                                LayoutType::Column => {
                                    cache.child_cross_sum(node)
                                },
                                LayoutType::Row => cache.child_cross_max(node),
                                LayoutType::Grid => cache.grid_col_max(node),
                            }
                    );
            
                    let max_cross = node.max_cross(store).unwrap_or_default().value_or(parent_cross, std::f32::MAX);
            
                    let border_left = node.border_left(store).unwrap_or_default().value_or(parent_main, 0.0);
                    let border_right = node.border_right(store).unwrap_or_default().value_or(parent_main, 0.0);
                    let border_top = node.border_top(store).unwrap_or_default().value_or(parent_main, 0.0);
                    let border_bottom = node.border_bottom(store).unwrap_or_default().value_or(parent_main, 0.0);
            
                    let position_type = node.position_type(store).unwrap_or_default();

                    // Parent overrides
                    match parent_layout_type {
                        LayoutType::Row | LayoutType::Column => {
                            if main_before == Units::Auto {
                                if cache.stack_first_child(node) {
                                    main_before = child_main_before;
                                } else {
                                    main_before = main_between;
                                }
                            }
            
                            if main_after == Units::Auto {
                                if cache.stack_last_child(node) {
                                    main_after = child_main_after;
                                }
                            }
                            
                            if cross_before == Units::Auto {
                                cross_before = child_cross_before;
                            }
            
                            if cross_after == Units::Auto {
                                cross_after = child_cross_after;
                            }
                        }
            
                        _=> {}
                    }

                    let mut new_main_before = 0.0;
                    let mut new_main = 0.0;
                    let mut new_main_after = 0.0;

                    let mut new_cross_before = 0.0;
                    let mut new_cross = 0.0;
                    let mut new_cross_after = 0.0;

                    let mut main_stretch_sum = 0.0;
                    let mut cross_stretch_sum = 0.0;

                    let mut main_free_space = parent_main;
                    let mut cross_free_space = parent_cross;


                    // TODO - replace all these match' with a function
                    match main_before {
                        Units::Pixels(val) => {
                            new_main_before = val.clamp(min_main_before, max_main_before);
                            main_free_space -= new_main_before;
                        }

                        Units::Percentage(val) => {
                            new_main_before = (val/100.0) * parent_main;
                            new_main_before = new_main_before.clamp(min_main_before, max_main_before);
                            main_free_space -= new_main_before;
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                            main_axis.push(ComputedData {
                                node: node.clone(),
                                value: val,
                                min: min_main_before,
                                max: max_main_before,
                                axis: Axis::Before,
                            });                        
                        }

                        _ => {}
                    }

                    match main {
                        Units::Pixels(val) => {
                            new_main = val.clamp(min_main, max_main);
                            main_free_space -= new_main;
                        }

                        Units::Percentage(val) => {
                            new_main = (val/100.0) * parent_main;
                            new_main = new_main.clamp(min_main, max_main);
                            main_free_space -= new_main;
                        }

                        Units::Stretch(val) => {
                            //if parent_layout_type == LayoutType::Column || (node.intrinsic_size(store, 0.0).is_none() && parent_layout_type == LayoutType::Row) {
                                main_stretch_sum += val;
                                main_axis.push(
                                    ComputedData {
                                        node: node.clone(),
                                        value: val,
                                        min: min_main,
                                        max: max_main,
                                        axis: Axis::Size,
                                    }
                                );
                            //}
                        }

                        Units::Auto => {
                            match layout_type {
                                LayoutType::Column => {
                                    new_main =
                                        cache.child_main_max(node);
                                }

                                LayoutType::Row | LayoutType::Grid=> {
                                    new_main =
                                        cache.child_main_sum(node);
                                }
                            }

                            new_main = new_main.clamp(min_main, max_main);

                            new_main += border_left + border_right;
                            main_free_space -= new_main;
                        }
                    }

                    match main_after {
                        Units::Pixels(val) => {
                            new_main_after = val.clamp(min_main_after, max_main_after);
                            main_free_space -= new_main_after;
                        }

                        Units::Percentage(val) => {
                            new_main_after = (val/100.0) * parent_main;
                            new_main_after = new_main_after.clamp(min_main_after, max_main_after);
                            main_free_space -= new_main_after;
                        }

                        Units::Stretch(val) => {
                            main_stretch_sum += val;
                            main_axis.push(
                                ComputedData {
                                    node: node.clone(),
                                    value: val,
                                    min: min_main_after,
                                    max: max_main_after,
                                    axis: Axis::After,
                                }
                            );
                        }

                        _ => {}
                    }

                    match cross_before {
                        Units::Pixels(val) => {
                            new_cross_before = val.clamp(min_cross_before, max_cross_before);
                            cross_free_space -= new_cross_before;
                        }

                        Units::Percentage(val) => {
                            new_cross_before = (val/100.0) * parent_cross;
                            new_cross_before = new_cross_before.clamp(min_cross_before, max_cross_before);
                            cross_free_space -= new_cross_before;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                            cross_axis.push(
                                ComputedData {
                                    node: node.clone(),
                                    value: val,
                                    min: min_cross_before,
                                    max: max_cross_before,
                                    axis: Axis::Before,
                                }
                            );
                        }

                        _ => {}
                    }

                    match cross {
                        Units::Pixels(val) => {
                            new_cross = val.clamp(min_cross, max_cross);
                            cross_free_space -= new_cross;
                        }

                        Units::Percentage(val) => {
                            new_cross = (val/100.0) * parent_cross;
                            new_cross = new_cross.clamp(min_cross, max_cross);
                            cross_free_space -= new_cross;
                        }

                        Units::Stretch(val) => {
                            //if parent_layout_type == LayoutType::Row || (node.intrinsic_size(store, 0.0).is_none() && parent_layout_type == LayoutType::Column) {
                                cross_stretch_sum += val;
                                cross_axis.push(
                                    ComputedData {
                                        node: node.clone(),
                                        value: val,
                                        min: min_cross,
                                        max: max_cross,
                                        axis: Axis::Size,
                                    }
                                );
                            //}
                        }

                        Units::Auto => {
                            match layout_type {
                                LayoutType::Column | LayoutType::Grid => {
                                    new_cross =
                                        cache.child_cross_sum(node);
                                }

                                LayoutType::Row => {
                                    new_cross =
                                        cache.child_cross_max(node);
                                }
                            }

                            new_cross = new_cross.clamp(min_cross, max_cross);

                            new_cross += border_top + border_bottom;
                            cross_free_space -= new_cross;
                        }
                    }

                    match cross_after {
                        Units::Pixels(val) => {
                            new_cross_after = val.clamp(min_cross_after, max_cross_after);
                            cross_free_space -= val;
                        }

                        Units::Percentage(val) => {
                            new_cross_after = (val/100.0) * parent_cross;
                            new_cross_after = new_cross_after.clamp(min_cross_after, max_cross_after);
                            cross_free_space -= new_cross_after;
                        }

                        Units::Stretch(val) => {
                            cross_stretch_sum += val;
                            cross_axis.push(
                                ComputedData {
                                    node: node.clone(),
                                    value: val,
                                    min: min_cross_after,
                                    max: max_cross_after,
                                    axis: Axis::After,
                                }
                            );
                        }

                        _ => {}
                    }

                    cache.set_new_main(node, new_main);
                    cache.set_new_cross(node, new_cross);
                    cache.set_main_before(node, new_main_before);
                    cache.set_main_after(node, new_main_after);
                    cache.set_cross_before(node, new_cross_before);
                    cache.set_cross_after(node, new_cross_after);
                
                    if position_type == PositionType::ParentDirected {
                        parent_cross_free_space -= parent_cross - cross_free_space;
                        parent_main_free_space -= parent_main - main_free_space;
                        parent_cross_stretch_sum += cross_stretch_sum;
                        parent_main_stretch_sum += main_stretch_sum;
                    }

                    cache
                        .set_main_free_space(node, main_free_space);
                    cache
                        .set_main_stretch_sum(node, main_stretch_sum);
                    cache
                        .set_cross_free_space(node, cross_free_space);
                    cache
                        .set_cross_stretch_sum(node, cross_stretch_sum);
                

                    
                
                }



                if parent_main_stretch_sum == 0.0 {
                    parent_main_stretch_sum = 1.0;
                }

                if parent_cross_stretch_sum == 0.0 {
                    parent_cross_stretch_sum = 1.0;
                }

                // Sort the stretch elements in each axis by the maximum size
                main_axis.sort_by(|a, b| a.max.partial_cmp(&b.max).unwrap());
                cross_axis.sort_by(|a, b| a.max.partial_cmp(&b.max).unwrap());

                let mut main_stretch_sum = 0.0;
                let mut main_free_space = 0.0;
                let mut cross_stretch_sum = 0.0;
                let mut cross_free_space = 0.0;


                ////////////////////////////////////////////
                // Calculate flexible Column space & size //
                ////////////////////////////////////////////
                for computed_data in cross_axis.iter() {

                    let node = computed_data.node.clone();

                    let position_type = node.position_type(store).unwrap_or_default();

                    match position_type {
                        PositionType::SelfDirected => {
                            cross_free_space = cache.cross_free_space(node);
                            cross_stretch_sum = cache.cross_stretch_sum(node);
                        }

                        PositionType::ParentDirected => {
                            //match parent_layout_type {
                                //LayoutType::Column => {
                                    main_stretch_sum = parent_main_stretch_sum;
                                    main_free_space = parent_main_free_space;
                                //}

                                //LayoutType::Row => {
                                    cross_free_space = cache.cross_free_space(node);
                                    cross_stretch_sum = cache.cross_stretch_sum(node);
                                //}     
                                
                                //_=> {}
                            //}

                        }
                    }

                    if cross_stretch_sum == 0.0 {
                        cross_stretch_sum = 1.0;
                    }

                    #[cfg(feature = "rounding")]
                    let mut new_value = (cross_free_space * computed_data.value / vertical_stretch_sum).round();
                    #[cfg(not(feature = "rounding"))]
                    let mut new_value = cross_free_space * computed_data.value / cross_stretch_sum;

                    new_value = new_value.clamp(computed_data.min, computed_data.max);

                    match computed_data.axis {
                        Axis::Before => {
                            cache.set_cross_before(node, new_value);
                        }

                        Axis::Size => {
                            cache.set_new_cross(node, new_value);
                            // if let Some(intrinsic_size) = node.intrinsic_size(store, new_value) {
                            //     if parent_layout_type == LayoutType::Row {
                            //         parent_horizontal_free_space -= intrinsic_size;
                            //         cache.set_new_width(node, intrinsic_size);
                            //     }
                            // }
                        }

                        Axis::After => {
                            cache.set_cross_after(node, new_value);
                        }
                    }

                    match position_type {
                        PositionType::SelfDirected => {
                            cache.set_cross_stretch_sum(node, cross_stretch_sum - computed_data.value);
                            cache.set_cross_free_space(
                                node,
                                cross_free_space - new_value,
                            );
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Row => {
                                    cache.set_cross_stretch_sum(
                                        node,
                                        cross_stretch_sum - computed_data.value,
                                    );
                                    cache.set_cross_free_space(
                                        node,
                                        cross_free_space - new_value,
                                    );
                                }

                                LayoutType::Column => {
                                    parent_cross_free_space -= new_value;
                                    parent_cross_stretch_sum -= computed_data.value;
                                }

                                _ => {}
                            };
                        }
                    }
                }
                
                
                ////////////////////////////////////////
                // Calculate flexible Row space & size //
                /////////////////////////////////////////
                for computed_data in main_axis.iter() {
                    
                    let node = computed_data.node.clone();

                    let position_type = node.position_type(store).unwrap_or_default();

                    match position_type {
                        PositionType::SelfDirected => {
                            main_free_space = cache.main_free_space(node);
                            main_stretch_sum = cache.main_stretch_sum(node);
                        }

                        PositionType::ParentDirected => {
                            //match parent_layout_type {
                                //LayoutType::Row => {
                                    main_stretch_sum = parent_main_stretch_sum;
                                    main_free_space = parent_main_free_space;
                                //}

                                //LayoutType::Column => {
                                    cross_free_space = cache.cross_free_space(node);
                                    cross_stretch_sum = cache.cross_stretch_sum(node);
                                //}

                                //_=> {}
                            //}
                            
                        }
                    }

                    println!("{:?} Main free space: {}", node, main_free_space);

                    // Prevent a divide by zero when the stretch sum is 0
                    if main_stretch_sum == 0.0 {
                        main_stretch_sum = 1.0;
                    }

                    // Compute the new left/width/height based on free space, stretch factor, and stretch_sum
                    #[cfg(feature = "rounding")]
                    let mut new_value = (main_free_space * computed_data.value / main_stretch_sum).round();
                    #[cfg(not(feature = "rounding"))]
                    let mut new_value = main_free_space * computed_data.value / main_stretch_sum;

                    // Clamp the new left/width/right to be between min_ left/width/right and max_ left/width/right
                    new_value = new_value.clamp(computed_data.min, computed_data.max);

                    // Could perhaps replace this with a closure
                    match computed_data.axis {
                        Axis::Before => {
                            cache.set_main_before(node, new_value);
                        }

                        Axis::Size => {
                            cache.set_new_main(node, new_value);
                        }

                        Axis::After => {
                            cache.set_main_after(node, new_value);
                        }
                    }

                    match position_type {
                        PositionType::SelfDirected => {
                            cache.set_main_stretch_sum(node, main_stretch_sum - computed_data.value);
                            cache.set_main_free_space(
                                node,
                                main_free_space - new_value,
                            );
                        }

                        PositionType::ParentDirected => {
                            match parent_layout_type {
                                LayoutType::Column => {
                                    cache.set_main_stretch_sum(
                                        node,
                                        main_stretch_sum - computed_data.value,
                                    );
                                    cache.set_main_free_space(
                                        node,
                                        main_free_space - new_value,
                                    );
                                }

                                LayoutType::Row => {
                                    parent_main_free_space -= new_value;
                                    parent_main_stretch_sum -= computed_data.value;
                                }

                                _ => {}
                            };
                        }
                    }
                }

                let mut current_posx = 0.0;
                let mut current_posy = 0.0;

                let parent_posx = cache.posx(parent) + parent_border_left;
                let parent_posy = cache.posy(parent) + parent_border_top;

                ///////////////////////
                // Position Children //
                ///////////////////////
                for node in hierarchy.child_iter(parent) {

                    let visible = cache.visible(node);
                    if !visible {
                        continue;
                    }

                    // let main_before = cache.main_before(node);
                    // let main_after = cache.main_after(node);
                    // let cross_before = cache.cross_before(node);
                    // let cross_after = cache.cross_after(node);

                    // let new_main = cache.new_main(node);
                    // let new_cross = cache.new_cross(node);

                    let (new_left, new_width, new_right, new_top, new_height, new_bottom) = match parent_layout_type {
                        LayoutType::Row => {
                            (cache.main_before(node), cache.new_main(node), cache.main_after(node), cache.cross_before(node), cache.new_cross(node), cache.cross_after(node))
                        }

                        LayoutType::Column | LayoutType::Grid => {
                            (cache.cross_before(node), cache.new_cross(node), cache.cross_after(node), cache.main_before(node), cache.new_main(node), cache.main_after(node))
                        }
                    };

                    let position_type = node.position_type(store).unwrap_or_default();

                    let (new_posx, new_posy) = match position_type {
                        PositionType::SelfDirected => {
                            (parent_posx + new_left, parent_posy + new_top)
                        }

                        PositionType::ParentDirected => {
                            let new_posx = parent_posx + current_posx + new_left;
                            let new_posy = parent_posy + current_posy + new_top;

                            match parent_layout_type {
                                LayoutType::Column => {
                                    current_posy += new_top + new_height + new_bottom;
                                }

                                LayoutType::Row => {
                                    current_posx += new_left + new_width + new_right;
                                }

                                _ => {}
                            }

                            (new_posx, new_posy)
                        }
                    };
                    

                    if new_posx != cache.posx(node) {
                        cache.set_geo_changed(node, GeometryChanged::POSX_CHANGED, true);
                    }

                    if new_posy != cache.posy(node) {
                        cache.set_geo_changed(node, GeometryChanged::POSY_CHANGED, true);
                    }

                    if new_width != cache.width(node) {
                        cache.set_geo_changed(node, GeometryChanged::WIDTH_CHANGED, true);
                    }

                    if new_height != cache.height(node) {
                        cache.set_geo_changed(node, GeometryChanged::HEIGHT_CHANGED, true);
                    }

                    cache.set_posx(node, new_posx);
                    cache.set_posy(node, new_posy);
                    cache.set_width(node, new_width);
                    cache.set_height(node, new_height);
                }
            }

            LayoutType::Grid => {
                /////////////////////////////////////////////////////
                // Determine Size of non-flexible rows and columns //
                /////////////////////////////////////////////////////
                let grid_rows = parent.grid_rows(store).unwrap_or_default();
                let grid_cols = parent.grid_cols(store).unwrap_or_default();

                let mut row_heights = vec![(0.0, 0.0); 2*grid_rows.len() + 2];
                let mut col_widths = vec![(0.0, 0.0,); 2*grid_cols.len() + 2];

                let row_heights_len = row_heights.len();
                let col_widths_len = col_widths.len();

                let mut col_free_space = parent_main;
                let mut row_free_space = parent_cross;

                let mut row_stretch_sum = 0.0;
                let mut col_stretch_sum = 0.0;

                let main_between = parent.main_between(store).unwrap_or_default();
                let cross_between = parent.cross_between(store).unwrap_or_default();

                let child_left = parent.child_main_before(store).unwrap_or_default();
                let child_right = parent.child_main_after(store).unwrap_or_default();
                let child_top = parent.child_cross_before(store).unwrap_or_default();
                let child_bottom = parent.child_cross_after(store).unwrap_or_default();

                match child_top {
                    Units::Pixels(val) => {
                        row_heights[0].1 = val;
                        row_free_space -= val;
                    }

                    Units::Stretch(val) => {
                        row_stretch_sum += val;
                    }

                    _=> {}
                }

                match child_bottom {
                    Units::Pixels(val) => {
                        row_heights[row_heights_len - 1].1 = val;
                        row_free_space -= val;
                    }

                    Units::Stretch(val) => {
                        row_stretch_sum += val;
                    }

                    _=> {}
                }

                match child_left {
                    Units::Pixels(val) => {
                        col_widths[0].1 = val;
                        col_free_space -= val;
                    }

                    Units::Stretch(val) => {
                        col_stretch_sum += val;
                    }

                    _=> {}
                }

                match child_right {
                    Units::Pixels(val) => {
                        col_widths[col_widths_len - 1].1 = val;
                        col_free_space -= val;
                    }

                    Units::Stretch(val) => {
                        col_stretch_sum += val;
                    }

                    _=> {}
                }

                for (i, row) in grid_rows.iter().enumerate() {

                    let row_index = 2 * i + 1;
    
                    match row {
                        &Units::Pixels(val) => {
                            row_heights[row_index].1 = val;
                            row_free_space -= val;
                        }

                        &Units::Stretch(val) => {
                            row_stretch_sum += val;
                        }

                        _ => {}
                    }

                    if i < grid_rows.len() - 1 {
                        let gutter_index = 2*i + 2;
                        match main_between {
                            Units::Pixels(val) => {
                                row_heights[gutter_index].1 = val;
                                row_free_space -= val;
                            }

                            Units::Stretch(val) => {
                                row_stretch_sum += val;
                            }

                            _=> {}
                        }
                    }
                }

                for (i, col) in grid_cols.iter().enumerate() {
                    let col_index = 2*i + 1;
                    match col {
                        &Units::Pixels(val) => {
                            col_widths[col_index].1 = val;
                            col_free_space -= val;
                        }

                        &Units::Stretch(val) => {
                            col_stretch_sum += val;
                        }

                        _ => {}
                    }

                    if i < grid_cols.len() - 1 {
                        let gutter_index = 2*i + 2;
                        match cross_between {
                            Units::Pixels(val) => {
                                col_widths[gutter_index].1 = val;
                                col_free_space -= val;
                            }

                            Units::Stretch(val) => {
                                col_stretch_sum += val;
                            }

                            _=> {}
                        }
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

                match child_top {

                    Units::Stretch(val) => {
                        row_heights[0].1 = row_free_space * val / row_stretch_sum;
                    }

                    _=> {}
                }

                match child_bottom {

                    Units::Stretch(val) => {
                        row_heights[row_heights_len - 1].1 = row_free_space * val / row_stretch_sum;
                    }

                    _=> {}
                }

                match child_left {

                    Units::Stretch(val) => {
                        col_widths[0].1 = col_free_space * val / col_stretch_sum;
                    }

                    _=> {}
                }

                match child_right {

                    Units::Stretch(val) => {
                        col_widths[col_widths_len - 1].1 = col_free_space * val / col_stretch_sum;
                    }

                    _=> {}
                }


                let mut current_row_pos = cache.posy(parent) + row_heights[0].1;
                let mut current_col_pos = cache.posx(parent) + col_widths[0].1;

                for (i, row) in grid_rows.iter().enumerate() {
                    
                    let row_index = 2*i + 1;
                    match row {
                        &Units::Stretch(val) => {
                            row_heights[row_index].1 = row_free_space * val / row_stretch_sum;
                        }

                        _ => {}
                    }

                    row_heights[row_index].0 = current_row_pos;
                    current_row_pos += row_heights[row_index].1;

                    if i < grid_rows.len() - 1 {
                        let gutter_index = 2*i + 2;
                        match main_between {
                            Units::Stretch(val) => {
                                row_heights[gutter_index].1 = row_free_space * val / row_stretch_sum;
                            }

                            _=> {}
                        }

                        row_heights[gutter_index].0 = current_row_pos;
                        current_row_pos += row_heights[gutter_index].1;
                    }

                    
                }
                let row_heights_len = row_heights.len() - 1;
                row_heights[row_heights_len - 1].0 = current_row_pos;

                for (i, col) in grid_cols.iter().enumerate() {
                    let col_index = 2*i + 1;
                    
                    match col {
                        &Units::Stretch(val) => {
                            col_widths[col_index].1 = col_free_space * val / col_stretch_sum;
                        }

                        _ => {}
                    }

                    col_widths[col_index].0 = current_col_pos;
                    current_col_pos += col_widths[col_index].1;

                    if i < grid_cols.len() - 1 {
                        let gutter_index = 2*i + 2;
                        match cross_between {
                            Units::Stretch(val) => {
                                col_widths[gutter_index].1 = col_free_space * val / col_stretch_sum;
                            }

                            _=> {}
                        }

                        col_widths[gutter_index].0 = current_col_pos;
                        current_col_pos += col_widths[gutter_index].1;
                    }

                    
                }

                let col_widths_len = col_widths.len() - 1;
                col_widths[col_widths_len - 1].0 = current_col_pos;

                ///////////////////////////////////////////////////
                // Position and Size child nodes within the grid //
                ///////////////////////////////////////////////////
                for node in hierarchy.child_iter(parent) {

                    let visible = cache.visible(node);
                    if !visible {
                        continue;
                    }

                    let row_start = 2 * node.row_index(store).unwrap_or_default() + 1;
                    let row_span = 2 * node.row_span(store).unwrap_or(1) - 1;
                    let row_end = row_start + row_span;


                    let col_start = 2 * node.col_index(store).unwrap_or_default() + 1;
                    let col_span = 2 * node.col_span(store).unwrap_or(1) - 1;
                    let col_end = col_start + col_span;

                    let new_posx = col_widths[col_start].0;
                    let new_width = col_widths[col_end].0 - new_posx;

                    let new_posy = row_heights[row_start].0;
                    let new_height = row_heights[row_end].0 - new_posy;

                    if new_posx != cache.posx(node) {
                        cache.set_geo_changed(node, GeometryChanged::POSX_CHANGED, true);
                    }

                    if new_posy != cache.posy(node) {
                        cache.set_geo_changed(node, GeometryChanged::POSY_CHANGED, true);
                    }

                    if new_width != cache.width(node) {
                        cache.set_geo_changed(node, GeometryChanged::WIDTH_CHANGED, true);
                    }

                    if new_height != cache.height(node) {
                        cache.set_geo_changed(node, GeometryChanged::HEIGHT_CHANGED, true);
                    }

                    cache.set_posx(node, new_posx);
                    cache.set_posy(node, new_posy);
                    cache.set_width(node, new_width);
                    cache.set_height(node, new_height);

                    cache.set_new_main(node, cache.width(node));
                    cache.set_new_cross(node, cache.height(node));
                }
            }
        }
    }
}