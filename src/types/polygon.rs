use super::{GeometryImpl, VectorMatrix, MBR};

pub type PolygonCoordinates<const N: usize, U> = VectorMatrix<N, U>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Un polygone
pub struct Polygon<const N: usize, U> {
    pub coordinates: PolygonCoordinates<N, U>,
    pub srid: Option<u32>,
}

impl<const N: usize, U> GeometryImpl for Polygon<N, U>
where
    U: Clone + PartialEq,
{
    type Coordinates = PolygonCoordinates<N, U>;

    fn new<C: Into<Self::Coordinates>>(coordinates: C) -> Self {
        let mut coordinates: PolygonCoordinates<N, U> = coordinates.into();

        // Ensure we close all rings.
        coordinates.iter_mut().for_each(|v| v.close_ring());

        Self {
            coordinates,
            srid: None,
        }
    }
}

impl<const N: usize, U> Polygon<N, U>
where
    U: Copy + PartialOrd,
{
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.min_x(),
            max_x: self.coordinates.max_x(),
            min_y: self.coordinates.min_y(),
            max_y: self.coordinates.max_y(),
        }
    }
}
