use raylib::math::Vector3;

use crate::rendering::Ray;

use super::{HitData, Triangle, AABB};

pub struct BVHNode {
    aabb: AABB,
    left: usize,
    right: usize,
    leaf: bool,
    first: usize,
    tris: usize,
}

impl Clone for BVHNode {
    fn clone(&self) -> Self {
        BVHNode {
            aabb: self.aabb.clone(),
            right: self.right,
            left: self.left,
            leaf: self.leaf,
            first: self.first,
            tris: self.tris,
        }
    }
}

pub struct BVH<'a> {
    nodes: Vec<Option<BVHNode>>,
    tris: &'a mut [Triangle],
}

impl<'a> BVH<'a> {
    pub fn new(tris: &'a mut [Triangle]) -> Self {
        Self {
            nodes: (0..(2 * tris.len() - 1)).map(|_| None).collect(),
            tris,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitData> {
        if let Some(node) = &self.nodes[0] {
            if !node.aabb.intersect(ray) {
                return None;
            }
        }

        None
    }

    pub fn build(&mut self) {
        let aabb = AABB::from_tris(self.tris);

        let root = BVHNode {
            aabb,
            left: 1,
            right: 2,
            leaf: true,
            first: 0,
            tris: self.tris.len(),
        };

        self.nodes[0] = Some(root);

        self.subdivide(0);
    }

    fn subdivide(&mut self, node_idx: usize) {
        if let Some(node) = &mut self.nodes[node_idx] {
            let extent = node.aabb.max - node.aabb.min;

            enum Axis {
                X,
                Y,
                Z,
            }

            let split: (Axis, f32) = if extent.x >= extent.y && extent.x >= extent.z {
                (Axis::X, node.aabb.min.x + 0.5 * extent.x)
            } else if extent.y >= extent.x && extent.y >= extent.z {
                (Axis::Y, node.aabb.min.y + 0.5 * extent.y)
            } else {
                (Axis::Z, node.aabb.min.z + 0.5 * extent.z)
            };

            let mut i = 0;
            let mut j = i + node.tris - 1;

            while i <= j {
                match split {
                    (Axis::X, pos) => {
                        if self.tris[i].centroid().x < pos {
                            i += 1;
                        } else {
                            let tmp = self.tris[i].clone();
                            self.tris[i] = self.tris[j].clone();
                            self.tris[j] = tmp;
                            j -= 1;
                        }
                    }
                    (Axis::Y, pos) => {
                        if self.tris[i].centroid().y < pos {
                            i += 1;
                        } else {
                            let tmp = self.tris[i].clone();
                            self.tris[i] = self.tris[j].clone();
                            self.tris[j] = tmp;
                            j -= 1;
                        }
                    }
                    (Axis::Z, pos) => {
                        if self.tris[i].centroid().z < pos {
                            i += 1;
                        } else {
                            let tmp = self.tris[i].clone();
                            self.tris[i] = self.tris[j].clone();
                            self.tris[j] = tmp;
                            j -= 1;
                        }
                    }
                }
            }

            let left_count = i - node.first;
            if left_count == 0 || left_count == node.tris {
                return;
            }

            node.leaf = false;

            let left_idx = 2 * node_idx + 1;
            let right_idx = 2 * node_idx + 2;

            let left = BVHNode {
                left: 2 * left_idx + 1,
                right: 2 * left_idx + 2,
                aabb: AABB::from_tris(&self.tris[node.first..(node.first + left_count)]),
                leaf: true,
                first: node.first,
                tris: left_count,
            };

            let right = BVHNode {
                left: 2 * right_idx + 1,
                right: 2 * right_idx + 2,
                aabb: AABB::from_tris(&self.tris[i..(i + node.tris - left_count)]),
                leaf: true,
                first: i,
                tris: node.tris - left_count,
            };

            self.nodes[left_idx] = Some(left);
            self.nodes[right_idx] = Some(right);

            self.subdivide(left_idx);
            self.subdivide(right_idx);
        }
    }
}
