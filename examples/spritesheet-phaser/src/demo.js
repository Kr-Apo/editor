import {parseSheet} from './metadata.js';
const reportError=error=>{document.querySelector('#error').textContent=String(error);window.demoError=String(error);};
try{
  const get=async url=>{const r=await fetch(url);if(!r.ok)throw new Error(`Cannot load ${url}: ${r.status}`);return r.json();};
  const manifest=await get('./assets/manifest.json');
  const clips=await Promise.all(manifest.map(async item=>({...item,references:item.references??[],sheet:parseSheet(await get(item.metadata))})));
  const pageKey=(id,index)=>`${id}/page/${index}`;
  const refKey=(id,index)=>`${id}/reference/${index}`;
  let scene,active=clips[0],sprite,reference,loops=0,updates=0,seen=new Set(),repeatTimes=[];
  const indexFor=name=>active.sheet.frames.find(frame=>frame.name===name)?.index;
  const game=new Phaser.Game({type:Phaser.WEBGL,parent:'stage',width:1100,height:600,transparent:true,banner:false,
    audio:{noAudio:true},render:{preserveDrawingBuffer:true,antialias:true,roundPixels:true},
    scene:{
      preload(){
        this.load.on('loaderror',file=>reportError(`Missing resource: ${file.url}`));
        for(const clip of clips){
          const base=new URL(clip.metadata,location.href);
          for(const page of clip.sheet.pages)this.load.image(pageKey(clip.id,page.index),new URL(page.image,base).href);
          for(const ref of clip.references)this.load.image(refKey(clip.id,ref.index),ref.image);
        }
      },
      create(){
        try{
          scene=this;
          for(const clip of clips){
            for(const frame of clip.sheet.frames){
              const r=frame.rect;this.textures.get(pageKey(clip.id,frame.page)).add(frame.name,0,r.x,r.y,r.w,r.h);
            }
            this.anims.create({key:clip.id,frames:clip.sheet.frames.map(frame=>({key:pageKey(clip.id,frame.page),frame:frame.name})),frameRate:clip.sheet.fps,repeat:clip.sheet.looping?-1:0});
          }
          sprite=this.add.sprite(550,300,pageKey(active.id,active.sheet.frames[0].page),active.sheet.frames[0].name).setOrigin(0.5);
          reference=this.add.image(825,300,pageKey(active.id,active.sheet.frames[0].page),active.sheet.frames[0].name).setOrigin(0.5).setVisible(false);
          sprite.on('animationupdate',(_animation,frame)=>{updates++;seen.add(indexFor(frame.textureFrame));});
          sprite.on('animationrepeat',()=>{loops++;repeatTimes.push(performance.now());});
          window.sheetDemo={game,clips:clips.map(c=>c.id),select,play,pause,showFrame,compare,
            state:()=>({clip:active.id,index:indexFor(sprite.frame.name),loops,updates,seen:[...seen],repeatTimes:[...repeatTimes],fps:sprite.anims.currentAnim?.frameRate,duration:sprite.anims.currentAnim?.duration,pages:active.sheet.pages.length,x:sprite.x,y:sprite.y,origin:[sprite.originX,sprite.originY],webgl:game.renderer.type===Phaser.WEBGL,phaser:Phaser.VERSION})};
          play();window.demoReady=true;
        }catch(error){reportError(error);}
      },
      update(){if(!sprite)return;document.querySelector('#status').textContent=`${active.sheet.animation} · ${active.sheet.fps} FPS · frame ${indexFor(sprite.frame.name)} / ${active.sheet.frame_count-1} · ${loops} boucle(s)`;}
    }});
  function select(id){active=clips.find(c=>c.id===id);if(!active)throw new Error('Unknown clip');document.querySelector('#clip').value=id;play();}
  function play(){
    loops=0;updates=0;seen=new Set([0]);repeatTimes=[];sprite.setPosition(550,300).setOrigin(active.sheet.origin.pivot.x,active.sheet.origin.pivot.y);
    reference.setVisible(false);sprite.play(active.id);document.querySelector('#left-label').textContent='Animation spritesheet';document.querySelector('#right-label').textContent='';
  }
  function pause(){sprite.anims.pause();}
  function showFrame(index){
    const frame=active.sheet.frames.find(f=>f.index===index);if(!frame)throw new Error('Missing frame');
    sprite.stop();sprite.setTexture(pageKey(active.id,frame.page),frame.name).setPosition(275,300).setOrigin(frame.pivot.x,frame.pivot.y);
    const ref=active.references.find(r=>r.index===index);reference.setVisible(!!ref);if(ref)reference.setTexture(refKey(active.id,index)).setOrigin(frame.pivot.x,frame.pivot.y);
    document.querySelector('#left-label').textContent=`Spritesheet · frame ${index}`;document.querySelector('#right-label').textContent=ref?'PNG individuel M004.1':'';
  }
  async function compare(index){
    if(!active.references.some(ref=>ref.index===index))throw new Error('No individual PNG reference for this frame');
    showFrame(index);await new Promise(done=>game.events.once('postrender',done));
    const frame=active.sheet.frames.find(f=>f.index===index),w=frame.source_size.w,h=frame.source_size.h;
    const canvas=document.createElement('canvas');canvas.width=1100;canvas.height=600;const ctx=canvas.getContext('2d',{willReadFrequently:true});ctx.drawImage(game.canvas,0,0);
    const a=ctx.getImageData(275-w/2,300-h/2,w,h).data,b=ctx.getImageData(825-w/2,300-h/2,w,h).data;
    let maxError=0,badPixels=0,transparent=0,visible=0;
    for(let i=0;i<a.length;i+=4){let error=0;for(let c=0;c<4;c++)error=Math.max(error,Math.abs(a[i+c]-b[i+c]));maxError=Math.max(maxError,error);if(error>2)badPixels++;if(a[i+3]===0)transparent++;else visible++;}
    return {clip:active.id,index,maxError,badPixels,transparent,visible,x:sprite.x,y:sprite.y,origin:[sprite.originX,sprite.originY]};
  }
  const picker=document.querySelector('#clip');
  for(const clip of clips){const option=document.createElement('option');option.value=clip.id;option.textContent=clip.label;picker.append(option);}
  picker.onchange=()=>select(picker.value);document.querySelector('#play').onclick=play;document.querySelector('#pause').onclick=pause;
  for(const button of document.querySelectorAll('[data-index]'))button.onclick=()=>showFrame(Number(button.dataset.index));
}catch(error){reportError(error);}
