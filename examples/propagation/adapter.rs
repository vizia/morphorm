use morphorm::{Alignment, Cache, LayoutType, LayoutWrap, Node, PositionType, Units};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NodeId(pub usize);

#[derive(Clone, Copy, Default)]
pub struct NodeStyle {
    pub layout_type: LayoutType,
    pub wrap: LayoutWrap,
    pub width: Units,
    pub height: Units,
    pub min_width: Option<Units>,
    pub min_height: Option<Units>,
    pub max_width: Option<Units>,
    pub max_height: Option<Units>,
    pub aspect_ratio: Option<f32>,
    pub padding_left: Units,
    pub padding_right: Units,
    pub padding_top: Units,
    pub padding_bottom: Units,
    pub gap: Units,
}

#[derive(Default)]
pub struct Tree {
    pub children: Vec<Vec<NodeId>>,
    pub parents: Vec<Option<NodeId>>,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default)]
pub struct LayoutCache(pub Vec<Bounds>);

impl Cache for LayoutCache {
    type Node = NodeId;
    fn width(&self, node: &NodeId) -> f32 {
        self.0[node.0].width
    }
    fn height(&self, node: &NodeId) -> f32 {
        self.0[node.0].height
    }
    fn posx(&self, node: &NodeId) -> f32 {
        self.0[node.0].x
    }
    fn posy(&self, node: &NodeId) -> f32 {
        self.0[node.0].y
    }
    fn set_bounds(&mut self, node: &NodeId, x: f32, y: f32, width: f32, height: f32) {
        self.0[node.0] = Bounds { x, y, width, height };
    }
}

impl Node for NodeId {
    type Store = Vec<NodeStyle>;
    type Tree = Tree;
    type ChildIter<'t> = std::slice::Iter<'t, NodeId>;
    type CacheKey = NodeId;
    type SubLayout<'a> = ();

    fn key(&self) -> NodeId {
        *self
    }
    fn children<'t>(&'t self, tree: &'t Tree) -> Self::ChildIter<'t> {
        tree.children[self.0].iter()
    }
    fn parent<'t>(&'t self, tree: &'t Tree) -> Option<&'t Self> {
        tree.parents[self.0].as_ref()
    }
    fn visible(&self, _store: &Self::Store) -> bool {
        true
    }
    fn layout_type(&self, store: &Self::Store) -> Option<LayoutType> {
        Some(store[self.0].layout_type)
    }
    fn position_type(&self, _store: &Self::Store) -> Option<PositionType> {
        Some(PositionType::Relative)
    }
    fn wrap(&self, store: &Self::Store) -> Option<LayoutWrap> {
        Some(store[self.0].wrap)
    }
    fn alignment(&self, _store: &Self::Store) -> Option<Alignment> {
        Some(Alignment::TopLeft)
    }
    fn width(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].width)
    }
    fn height(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].height)
    }
    fn aspect_ratio(&self, store: &Self::Store) -> Option<f32> {
        store[self.0].aspect_ratio
    }
    fn left(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn right(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn top(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn bottom(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn content_size(&self, _: &Self::Store, _: &mut (), _: Option<f32>, _: Option<f32>) -> Option<(f32, f32)> {
        None
    }
    fn padding_left(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_left)
    }
    fn padding_right(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_right)
    }
    fn padding_top(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_top)
    }
    fn padding_bottom(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].padding_bottom)
    }
    fn vertical_gap(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].gap)
    }
    fn horizontal_gap(&self, store: &Self::Store) -> Option<Units> {
        Some(store[self.0].gap)
    }
    fn min_vertical_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn min_horizontal_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn max_vertical_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn max_horizontal_gap(&self, _store: &Self::Store) -> Option<Units> {
        None
    }
    fn min_width(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].min_width
    }
    fn min_height(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].min_height
    }
    fn max_width(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].max_width
    }
    fn max_height(&self, store: &Self::Store) -> Option<Units> {
        store[self.0].max_height
    }
    fn grid_columns(&self, _store: &Self::Store) -> Option<Vec<Units>> {
        None
    }
    fn grid_rows(&self, _store: &Self::Store) -> Option<Vec<Units>> {
        None
    }
    fn column_start(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
    fn row_start(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
    fn column_span(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
    fn row_span(&self, _store: &Self::Store) -> Option<usize> {
        None
    }
}
