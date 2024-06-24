/**
 * Convert geometry file types
 */

use obj::{ Obj, Position };
use ply_rs::ply::{ Ply, DefaultElement, Encoding, ElementDef, PropertyDef, PropertyType, ScalarType, Property, Addable };

// https://github.com/Fluci/ply-rs/pull/23
pub async fn convert_obj_to_ply(_obj: &Obj<Position, u32>) -> Ply<DefaultElement> {
    let mut _ply = Ply::<DefaultElement>::new();
    // begin header
    _ply.header.encoding = Encoding::BinaryLittleEndian;
    // _ply.header.encoding = Encoding::Ascii;

    let mut v_e = ElementDef::new("vertex".to_string());
    v_e.properties.add(PropertyDef::new("x".to_string(), PropertyType::Scalar(ScalarType::Float)));
    v_e.properties.add(PropertyDef::new("y".to_string(), PropertyType::Scalar(ScalarType::Float)));
    v_e.properties.add(PropertyDef::new("z".to_string(), PropertyType::Scalar(ScalarType::Float)));
    _ply.header.elements.add(v_e);

    let mut f_e = ElementDef::new("face".to_string());
    f_e.properties.add(PropertyDef::new("vertex_indices".to_string(), PropertyType::List(ScalarType::UChar, ScalarType::UInt)));
    _ply.header.elements.add(f_e);
    // end header

    // add vertices
    let mut vertices = Vec::new();
    for obj_v in _obj.vertices.iter() {
        let mut v = DefaultElement::new();
        v.insert("x".to_string(), Property::Float(obj_v.position[0]));
        v.insert("y".to_string(), Property::Float(obj_v.position[1]));
        v.insert("z".to_string(), Property::Float(obj_v.position[2]));
        vertices.push(v);
    }
    
    // add faces
    let mut faces = Vec::new();
    for obj_f_ind in 0.._obj.indices.len()/3 {
        let mut f = DefaultElement::new();
        f.insert(
            "vertex_indices".to_string(), 
            Property::ListUInt(vec![
                _obj.indices[obj_f_ind * 3 + 0],
                _obj.indices[obj_f_ind * 3 + 1],
                _obj.indices[obj_f_ind * 3 + 2],
            ])
        );
        faces.push(f);
    }

    _ply.payload.insert("vertex".to_string(), vertices);
    _ply.payload.insert("face".to_string(), faces);
   
    _ply.make_consistent().unwrap();

    _ply
}