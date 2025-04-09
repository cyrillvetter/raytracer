use crate::primitive::{Aabb, Ray};
use crate::triangle::{Triangle, HitRecord};

use glam::Vec3;

pub const ROOT_IDX: usize = 0;

#[derive(Debug)]
pub struct Bvh {
    nodes: Vec<BvhNode>,
    triangles: Vec<Triangle>,
    nodes_used: usize,
}

#[derive(Debug, Clone)]
pub struct BvhNode {
    aabb: Aabb,
    left_child: usize,
    first_prim: usize,
    prim_count: usize
}

impl Default for BvhNode {
    fn default() -> Self {
        Self {
            aabb: Aabb::new(Vec3::INFINITY, Vec3::NEG_INFINITY),
            left_child: 0,
            first_prim: 0,
            prim_count: 0
        }
    }
}

impl BvhNode {
    pub fn update_bounds(&mut self, triangles: &Vec<Triangle>) {
        self.aabb = Aabb::new(Vec3::INFINITY, Vec3::NEG_INFINITY);

        for i in self.first_prim..self.first_prim+self.prim_count {
            let tri = &triangles[i];
            self.aabb.minimum = self.aabb.minimum.min(tri.v1.position);
            self.aabb.minimum = self.aabb.minimum.min(tri.v2.position);
            self.aabb.minimum = self.aabb.minimum.min(tri.v3.position);
            self.aabb.maximum = self.aabb.maximum.max(tri.v1.position);
            self.aabb.maximum = self.aabb.maximum.max(tri.v2.position);
            self.aabb.maximum = self.aabb.maximum.max(tri.v3.position);
        }
    }

    pub fn is_leaf(&self) -> bool {
        self.prim_count > 0
    }
}

impl Bvh {
    pub fn new(triangles: Vec<Triangle>) -> Self {
        let mut nodes: Vec<BvhNode> = vec![BvhNode::default(); 2 * triangles.len() - 1];
        let root = &mut nodes[ROOT_IDX];
        root.prim_count = triangles.len();
        root.update_bounds(&triangles);

        let mut bvh = Bvh {
            nodes,
            triangles,
            nodes_used: 1
        };

        bvh.subdivide(ROOT_IDX);
        println!("nodes used: {}", bvh.nodes_used);
        bvh
    }

    pub fn triangles_amount(&self) -> usize {
        self.triangles.len()
    }

    pub fn intersects(&self, ray: &Ray) -> Option<HitRecord> {
        self.traverse(ray, ROOT_IDX)
    }

    fn traverse(&self, ray: &Ray, node_idx: usize) -> Option<HitRecord> {
        let node = &self.nodes[node_idx];

        if node.is_leaf() {
            let mut nearest_dist = f32::INFINITY;
            let mut nearest_hit: Option<HitRecord> = None;

            for i in node.first_prim..node.first_prim+node.prim_count {
                let triangle = &self.triangles[i];

                match triangle.hit(&ray) {
                    Some(hit_record) if hit_record.t < nearest_dist => {
                        nearest_dist = hit_record.t;
                        nearest_hit = Some(hit_record);
                    },
                    _ => ()
                }
            }

            return nearest_hit;
        }

        if !node.aabb.hit(ray) {
            return None;
        }

        let left_hit = self.traverse(ray, node.left_child);
        let right_hit = self.traverse(ray, node.left_child + 1);

        match (left_hit, right_hit) {
            (Some(t1), Some(t2)) => if t1.t < t2.t {
                    Some(t1)
                } else {
                    Some(t2)
                },
            (Some(t1), None) => Some(t1),
            (None, Some(t2)) => Some(t2),
            (None, None) => None
        }
    }

    fn subdivide(&mut self, node_idx: usize) {
        let node = &mut self.nodes[node_idx];
        if node.prim_count <= 4 {
            return;
        }

        let extent = node.aabb.maximum - node.aabb.minimum;
        let mut axis = 0;
        if extent.y > extent.x {
            axis = 1;
        }

        if extent.z > extent[axis] {
            axis = 2;
        }

        let split_pos = node.aabb.minimum[axis] + extent[axis] * 0.5;

        let mut i = node.first_prim;
        let mut j = i + node.prim_count - 1;

        while i <= j {
            if self.triangles[i].centroid[axis] < split_pos {
                i += 1;
            } else {
                self.triangles.swap(i, j);
                j -= 1;
            }
        }

        let left_count = i - node.first_prim;
        if left_count == 0 || left_count == node.prim_count {
            return;
        }

        let left_child_idx = self.nodes_used;
        let right_child_idx = self.nodes_used + 1;
        self.nodes_used += 2;

        let node_first_prim = node.first_prim;
        let node_prim_count = node.prim_count;

        node.left_child = left_child_idx;
        node.prim_count = 0;

        self.nodes[left_child_idx].first_prim = node_first_prim;
        self.nodes[left_child_idx].prim_count = left_count;
        self.nodes[right_child_idx].first_prim = i;
        self.nodes[right_child_idx].prim_count = node_prim_count - left_count;

        self.nodes[left_child_idx].update_bounds(&self.triangles);
        self.nodes[right_child_idx].update_bounds(&self.triangles);

        self.subdivide(left_child_idx);
        self.subdivide(right_child_idx);
    }
}
