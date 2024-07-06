use std::ops::{Deref, DerefMut};

use super::{Vector, MBR};

/// Type représentant les coordonnées d'un point.
pub type PointCoordinates<const N: usize, U> = Vector<N, U>;

/// Un point dans un espace n-d.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Point<const N: usize, U> {
    pub coordinates: PointCoordinates<N, U>,
    pub srid: u32,
}

impl<const N: usize, U> Point<N, U> {
    pub fn new<V: Into<PointCoordinates<N, U>>>(coordinates: V) -> Self {
        Self {
            coordinates: coordinates.into(),
            srid: 4326,
        }
    }

    pub fn new_with_srid(coordinates: Vector<N, U>, srid: u32) -> Self {
        Self { coordinates, srid }
    }
}

impl<const N: usize, U> Point<N, U>
where
    U: Copy,
{
    pub fn mbr(&self) -> MBR<U> {
        MBR {
            min_x: self.coordinates.x(),
            max_x: self.coordinates.x(),
            min_y: self.coordinates.y(),
            max_y: self.coordinates.y(),
        }
    }
}

impl<const N: usize, U> Deref for Point<N, U> {
    type Target = [U; N];

    fn deref(&self) -> &Self::Target {
        self.coordinates.deref()
    }
}

impl<const N: usize, U> DerefMut for Point<N, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.coordinates.deref_mut()
    }
}

