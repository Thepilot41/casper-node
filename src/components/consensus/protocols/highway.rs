use crate::components::consensus::{
    consensus_protocol::{AddVertexOk, ProtocolState, VertexTrait},
    highway_core::{
        highway::{AddVertexOutcome, Highway},
        vertex::{Dependency, Vertex},
    },
    traits::Context,
};

impl<C: Context> VertexTrait for Vertex<C> {
    type Id = Dependency<C>;
    type Value = C::ConsensusValue;

    fn id(&self) -> Dependency<C> {
        self.id()
    }

    fn values(&self) -> Vec<C::ConsensusValue> {
        self.values().cloned().collect()
    }
}

impl<C: Context> ProtocolState for Highway<C> {
    type Error = String;
    type VId = Dependency<C>;
    type Vertex = Vertex<C>;

    fn add_vertex(&mut self, v: Vertex<C>) -> Result<AddVertexOk<Dependency<C>>, Self::Error> {
        let vid = v.id();
        match self.add_vertex(v) {
            AddVertexOutcome::Success => Ok(AddVertexOk::Success(vid)),
            AddVertexOutcome::MissingDependency(_vertex, dependency) => {
                Ok(AddVertexOk::MissingDependency(dependency))
            }
            AddVertexOutcome::Invalid(vertex) => Err(format!("invalid vertex: {:?}", vertex)),
        }
    }

    fn get_vertex(&self, v: Dependency<C>) -> Result<Option<Vertex<C>>, Self::Error> {
        Ok(self.get_dependency(v))
    }
}