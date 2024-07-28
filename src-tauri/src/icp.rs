use nalgebra::{Matrix4, Vector3};
use ndarray::prelude::*;
use align3d::pointcloud::PointCloud;
use align3d::icp::{ IcpParams, Icp };
use align3d::transform::Transform;
use align3d::io::read_ply;
use align3d::mesh::compute_normals;
use crate::db::get_pcd;

fn convert_raw_to_geometry(raw_vets: Vec<f32>, raw_norms: Vec<f32>) -> PointCloud {
    let points: Array1<Vector3<f32>> = Array1::from_shape_fn(raw_vets.len() / 3, |i| {
        Vector3::new(raw_vets[3 * i], raw_vets[3 * i + 1], raw_vets[3 * i + 2])
    });
    let normals: Array1<Vector3<f32>> = Array1::from_shape_fn(raw_norms.len() / 3, |i| {
        Vector3::new(raw_norms[3 * i], raw_norms[3 * i + 1], raw_norms[3 * i + 2])
    });
    PointCloud {
        points: points,
        normals: Some(normals),
        colors: None
    }
}

pub fn registration_for_raw(raw_vets: Vec<f32>, raw_norms: Vec<f32>, target_token: String) -> Matrix4<f32> {
    if let Some(target_pcd) = get_pcd(&target_token) {
        let source_pcd = convert_raw_to_geometry(raw_vets, raw_norms);
        let icp_params = IcpParams {
            max_iterations: 30,
            max_distance: 1.5,
            max_normal_angle: 120.0_f32.to_radians(),
            ..IcpParams::default()
        };
        let icp_inst = Icp::new(icp_params, &target_pcd);
    
        let transform = icp_inst.align(&source_pcd);
        return transform.0.to_matrix();
    }
    else {
        return Transform::eye().0.to_matrix();
    }
}

pub fn load_pcd_from_ply(filepath: &str) -> PointCloud {
    let geom = read_ply(filepath).unwrap();
    let normals = match geom.normals {
        Some(_normals) => _normals,
        None => compute_normals(&geom.points.view(), &geom.faces.unwrap().view()),
    };
    PointCloud {
        points: geom.points,
        normals: Some(normals),
        colors: None
    }
}