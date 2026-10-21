// Synthetic same-owner FFD source/destination with real weighted native assets.
#[allow(dead_code,reason="Shared accepted mesh/asset fixture; only helpers needed by each harness are used")]
mod base {include!(concat!(env!("CARGO_MANIFEST_DIR"),"/../document/tests/support/retrace_fixture.rs"));}
use ankhimate_core::{animation::{Animation,Interp,Key,Timeline},ids::{AnimationId,SkinId,SlotId},attachment::{Attachment,MeshAttachment}};
use ankhimate_document::{Document,commands::key_cmds::{KeyRef,TimelineAddr}};
use glam::{Vec2,vec2};
pub struct Rig{pub doc:Document,pub source:AnimationId,pub destination:AnimationId,pub skin:SkinId,pub slot:SlotId}
impl Rig{
 pub fn addr(&self)->TimelineAddr{TimelineAddr::Deform{slot:self.slot,attachment:"mesh".into(),owner:Some(self.skin)}}
 pub fn refs(&self,indices:&[usize])->Vec<KeyRef>{indices.iter().map(|i|KeyRef{addr:self.addr(),index:*i}).collect()}
 pub fn mesh_mut(&mut self)->&mut MeshAttachment{base::mesh_mut(&mut self.doc,self.skin,self.slot)}
 pub fn mesh(&self)->&MeshAttachment{base::mesh(&self.doc,self.skin,self.slot)}
}
pub fn fixture(weighted:bool)->Rig{
 let(mut d,skin,slot)=base::fixture(weighted);let n=if weighted{base::mesh(&d,skin,slot).influence_count()}else{base::mesh(&d,skin,slot).setup_vertices.len()};
 let keys=vec![Key{time:0.25,value:(0..n).map(|i|vec2(-(i as f32),0.)).collect(),interp:Interp::Stepped},Key{time:0.75,value:(0..n).map(|i|vec2(i as f32*2.,-3.+i as f32)).collect(),interp:Interp::Bezier{out_handle:vec2(0.25,-0.5),in_handle:vec2(0.75,1.5)}},Key::linear(1.25,(0..n).map(|i|vec2(0.,i as f32*0.1)).collect())];
 let mut source=Animation::new("FFD source",4.);source.timelines.push(Timeline::Deform{slot,attachment:"mesh".into(),keys});let source=d.animations.insert(source);
 let mut dest=Animation::new("FFD destination",4.);dest.timelines.push(Timeline::Deform{slot,attachment:"mesh".into(),keys:vec![Key::linear(0.,vec![Vec2::ZERO;n])]});let destination=d.animations.insert(dest);
 Rig{doc:d,source,destination,skin,slot}
}
pub fn deform(d:&Document,id:AnimationId)->&[Key<Vec<Vec2>>]{let Timeline::Deform{keys,..}=&d.animations[id].timelines[0]else{panic!("missing fixture FFD")};keys}
pub fn deform_mut(d:&mut Document,id:AnimationId)->&mut Vec<Key<Vec<Vec2>>>{let Timeline::Deform{keys,..}=&mut d.animations[id].timelines[0]else{panic!("missing fixture FFD")};keys}
pub fn another_mesh(r:&mut Rig)->TimelineAddr{let m=r.mesh().clone();r.doc.skeleton.skins[r.skin].set(r.slot,"other",Attachment::Mesh(m));TimelineAddr::Deform{slot:r.slot,attachment:"other".into(),owner:Some(r.skin)}}
