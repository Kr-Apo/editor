import {parseRig,PhaserRig} from '@ankhimate/phaser-runtime';

const stats=window.crapaudStats={ready:false,frames:0,loops:0,errors:[],missing:[],meshes:0,time:0};
const ui=id=>document.getElementById(id);
const fail=error=>{stats.errors.push(String(error?.message??error));ui('error').hidden=false;ui('error').textContent=stats.errors.at(-1);};
window.addEventListener('error',e=>fail(e.error??e.message));
window.addEventListener('unhandledrejection',e=>fail(e.reason));
let rig,game,playing=true,loop=true,last=0;
const get=async url=>{const r=await fetch(url);if(!r.ok)throw Error(`${url} (${r.status})`);return r.json();};
const [raw,sequence]=await Promise.all([get('assets/runtime/skeleton.json'),get('assets/png/sequence.json')]);
const data=parseRig(raw),duration=data.animations.frappe_sol.duration;
const show=time=>{
  const frame=Math.min(89,Math.floor((time+1e-7)*30));
  ui('reference').src=`assets/png/frame_${String(frame).padStart(6,'0')}.png`;
  ui('seek').value=frame;ui('clock').textContent=`${time.toFixed(2).replace('.',',')} / 3 s`;
  ui('phase').textContent=time<1?'Levée du sceptre':time<1.3?'Suspension':time<46/30?'Frappe':time<58/30?'Impact et rebond':'Retour au repos';
  stats.time=time;
};
const seek=time=>{rig.player.seek(time);rig.update(0);show(rig.player.time);};
game=new Phaser.Game({type:Phaser.WEBGL,width:512,height:512,parent:'game',transparent:true,banner:false,audio:{noAudio:true},render:{preserveDrawingBuffer:true},scene:{
  preload(){this.load.on('loaderror',file=>{stats.missing.push(file.key);fail(`Atlas manquant : ${file.key}`);});raw.atlas.pages.forEach((page,i)=>this.load.image(`pharaon-${i}`,`assets/runtime/${page.file}`));},
  create(){try{
    rig=new PhaserRig(this,data,{pageTextureKeys:raw.atlas.pages.map((_,i)=>`pharaon-${i}`)});
    const {center,zoom}=sequence.camera;
    rig.root.setPosition(256-center[0]*zoom,256+center[1]*zoom).setScale(zoom);
    rig.play('frappe_sol');
    stats.meshes=rig.root.list.filter(x=>x.type==='Mesh'&&x.visible).length;
    window.crapaud={game,rig,data,sequence,seek,pause(){playing=false;ui('pause').textContent='Lecture';},play(){playing=true;ui('pause').textContent='Pause';}};
    ui('restart').onclick=()=>{rig.play('frappe_sol');last=0;playing=true;ui('pause').textContent='Pause';show(0);};
    ui('pause').onclick=()=>{playing=!playing;ui('pause').textContent=playing?'Pause':'Lecture';};
    ui('loop').onchange=()=>{loop=ui('loop').checked;};
    ui('seek').oninput=()=>{playing=false;ui('pause').textContent='Lecture';seek(Number(ui('seek').value)/30);};
    stats.ready=true;show(0);
  }catch(e){fail(e);}},
  update(_time,delta){if(!stats.ready||stats.errors.length)return;try{
    if(playing){const before=rig.player.time;if(!loop&&before+delta/1000>=duration){seek((90-1)/30);playing=false;ui('pause').textContent='Lecture';}else{const pose=rig.update(delta);if(pose.bones.some(b=>Object.values(b.world).some(v=>!Number.isFinite(v))))throw Error('Pose non finie');if(rig.player.time<before)stats.loops++;show(rig.player.time);}}
    stats.frames++;last=rig.player.time;
  }catch(e){fail(e);}}
}});
