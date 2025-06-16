use svg::open;
use crate::core::cost::Cost;
use crate::core::entities::layout::Layout;
use crate::core::insertion::node_blueprint::NodeBlueprint;
use crate::optimization::instance::Instance;

///Representation of a layout that can be sent across threads

#[derive(Debug, Clone)]
pub struct SendableLayout {
    sheettype_id: usize,
    top_node: NodeBlueprint,
    cost: Cost,
    cut_count:u32,
    usage: f64,
}

impl SendableLayout {
    pub fn new(layout: &Layout) -> Self {
        let top_node =NodeBlueprint::from_node(*layout.top_node_index(), layout.nodes());
        let cut_count=top_node.calculate_cut_count();
        Self {
            sheettype_id: layout.sheettype().id(),
            top_node,
            cost: layout.cost_immut(false),
            cut_count,
            usage: layout.usage_immut(false),
        }
    }

    pub fn convert_to_layout(&self, _instance: &Instance) -> Layout {
        todo!();
    }

    pub fn sheettype_id(&self) -> usize {
        self.sheettype_id
    }
    pub fn top_node(&self) -> &NodeBlueprint {
        &self.top_node
    }
    pub fn cost(&self) -> &Cost {
        &self.cost
    }
    pub fn cut_count(&self) -> u32 {
        self.cut_count
    }
    pub fn usage(&self) -> f64 {
        self.usage
    }
}