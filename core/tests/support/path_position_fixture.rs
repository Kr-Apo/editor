use ankhimate_core::{animation::{Animation,Key,Timeline},attachment::{Attachment,PathAttachment},constraints::{Constraint,PathConstraint},ids::{BoneId,ConstraintId},skeleton::{Skeleton,Bone},slot::Slot};
use glam::{Vec2,vec2};
pub fn fixture(closed:bool,constant_speed:bool)->(Skeleton,Animation,ConstraintId,Vec<BoneId>){
 let mut s=Skeleton::new();let root=s.add_bone(bone("path",None));
 let slot=s.add_slot(Slot{attachment:Some("curve".into()),..Slot::new("path slot".into(),root)});
 let skin=s.default_skin;s.skins[skin].set(slot,"curve",Attachment::Path(PathAttachment{vertices:vec![Vec2::ZERO,vec2(100.,0.),vec2(100.,100.)],closed,constant_speed}));
 let mut bones=Vec::new();for i in 0..5 {let b=s.add_bone(bone(&format!("tail{i}"),bones.last().copied()));bones.push(b);}
 let mut path=PathConstraint::new("tail path",slot,bones.clone());path.spacing=0.2;
 let id=s.add_constraint(Constraint::Path(path));
 let mut a=Animation::new("progress",1.);a.looping=false;a.timelines=vec![Timeline::PathPosition{constraint:id,keys:vec![Key::linear(0.,0.),Key::linear(0.5,0.25),Key::linear(1.,0.5)]}];(s,a,id,bones)
}
pub fn bone(name:&str,parent:Option<BoneId>)->Bone{Bone{name:name.into(),parent,length:8.,local_transform:Default::default(),inherit:Default::default(),color:Bone::default_color()}}
