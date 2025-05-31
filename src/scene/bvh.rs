use crate::{math::Transform, rendering::Ray};

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

pub struct BVH {
    nodes: Vec<Option<BVHNode>>,
    tris: Vec<Triangle>,
    used_nodes: usize,
}

impl BVH {
    pub fn new(tris: Vec<Triangle>) -> Self {
        Self {
            nodes: (0..(2 * tris.len() - 1)).map(|_| None).collect(),
            tris,
            used_nodes: 0,
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<HitData> {
        self.intersect_node(ray, 0, 0)
    }

    fn intersect_node(&self, ray: &Ray, idx: usize, hits: u32) -> Option<HitData> {
        if let Some(node) = &self.nodes[idx] {
            if !node.aabb.intersect(ray) {
                return None;
            }

            if node.leaf {
                let tris = &self.tris[node.first..(node.first + node.tris)];

                if let Some((_, hit)) = tris.iter().fold(None, |acc, tri| match acc {
                    None => {
                        if let Some(hit) = tri.intersect(ray) {
                            Some((
                                ray.origin.distance_to(hit.p),
                                HitData {
                                    position: hit.p,
                                    normal: hit.normal,
                                    bary: hit.bary,
                                    node_hits: hits,
                                },
                            ))
                        } else {
                            None
                        }
                    }
                    Some((d, c_hit)) => {
                        if let Some(hit) = tri.intersect(ray) {
                            let nd = ray.origin.distance_to(hit.p);
                            if nd < d {
                                Some((
                                    nd,
                                    HitData {
                                        position: hit.p,
                                        normal: hit.normal,
                                        bary: hit.bary,
                                        node_hits: hits,
                                    },
                                ))
                            } else {
                                Some((d, c_hit))
                            }
                        } else {
                            Some((d, c_hit))
                        }
                    }
                }) {
                    return Some(hit);
                }
            } else {
                let left = self.intersect_node(ray, node.left, hits + 1);
                let right = self.intersect_node(ray, node.right, hits + 1);

                return match (left, right) {
                    (None, None) => None,
                    (Some(h), None) => Some(h),
                    (None, Some(h)) => Some(h),
                    (Some(h1), Some(h2)) => {
                        let d1 = h1.position.distance_to(ray.origin);
                        let d2 = h2.position.distance_to(ray.origin);

                        if d1 < d2 {
                            Some(h1)
                        } else {
                            Some(h2)
                        }
                    }
                };
            }
        }

        None
    }

    pub fn build(&mut self) {
        let aabb = AABB::from_tris(&self.tris);

        let root = BVHNode {
            aabb,
            left: 1,
            right: 2,
            leaf: true,
            first: 0,
            tris: self.tris.len(),
        };

        self.nodes[0] = Some(root);
        self.used_nodes += 1;

        self.subdivide(0);
    }

    fn subdivide(&mut self, node_idx: usize) {
        if let Some(node) = &mut self.nodes[node_idx] {
            if node.tris <= 2 {
                return;
            }
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

            let mut i = node.first;
            let mut j = i + node.tris - 1;

            while i <= j && j < self.tris.len() && i < self.tris.len() {
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

            let left_idx = self.used_nodes;
            let right_idx = self.used_nodes + 1;

            self.used_nodes += 2;

            let left = BVHNode {
                left: 0,
                right: 0,
                aabb: AABB::from_tris(&self.tris[node.first..(node.first + left_count)]),
                leaf: true,
                first: node.first,
                tris: left_count,
            };

            let right = BVHNode {
                left: 0,
                right: 0,
                aabb: AABB::from_tris(&self.tris[i..(i + node.tris - left_count)]),
                leaf: true,
                first: i,
                tris: node.tris - left_count,
            };

            node.left = left_idx;
            node.right = right_idx;

            self.nodes[left_idx] = Some(left);
            self.nodes[right_idx] = Some(right);

            self.subdivide(left_idx);
            self.subdivide(right_idx);
        }
    }
}
