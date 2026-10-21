#[allow(dead_code,reason="Shared native synthetic Path fixture")]
mod path_fixture {include!(concat!(env!("CARGO_MANIFEST_DIR"),"/../core/tests/support/path_position_fixture.rs"));}
pub fn path_document()->(ankhimate_document::Document,ankhimate_core::ids::AnimationId,ankhimate_core::ids::ConstraintId){
 use ankhimate_core::{attachment::{Attachment,RegionAttachment,Rect},assets::ImageAsset,slot::Slot};
 let (s,a,c,bones)=path_fixture::fixture(false,true);let mut d=ankhimate_document::Document::new();d.skeleton=s;let id=d.animations.insert(a);
 let mut png=std::io::Cursor::new(Vec::new());image::RgbaImage::from_pixel(4,4,image::Rgba([30,180,220,200])).write_to(&mut png,image::ImageFormat::Png).unwrap();d.assets.add(ImageAsset::new("tail art",png.into_inner(),4,4));
 let skin=d.skeleton.default_skin;for(i,b)in bones.iter().enumerate(){let slot=d.skeleton.add_slot(Slot{attachment:Some("art".into()),..Slot::new(format!("art{i}"),*b)});d.skeleton.skins[skin].set(slot,"art",Attachment::Region(RegionAttachment{texture:"tail art".into(),local_offset:glam::Vec2::ZERO,local_rotation:0.,local_scale:glam::Vec2::ONE,width:6.,height:6.,uv_rect:Rect{x:0.,y:0.,w:1.,h:1.},pivot:glam::Vec2::splat(0.5),sequence:None}));}
 (d,id,c)
}
