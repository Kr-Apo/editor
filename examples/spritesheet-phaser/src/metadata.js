// A small consumer of the documented spritesheet v1 contract. No runtime dependency.
export function parseSheet(raw) {
  const valid=(condition,message)=>{if(!condition)throw new Error(`Spritesheet: ${message}`);};
  const positive=value=>Number.isInteger(value)&&value>0;
  valid(raw?.spritesheet_version===1,'unsupported format version');
  valid(positive(raw.fps)&&raw.fps<=240&&positive(raw.frame_count),'invalid timing');
  valid(Number.isFinite(raw.duration)&&raw.duration>0&&typeof raw.looping==='boolean','invalid clip');
  valid(raw.alpha==='straight'&&raw.trim===false&&raw.rotated===false,'only untrimmed, unrotated straight RGBA is supported');
  valid(Number.isInteger(raw.padding)&&raw.padding>=0&&raw.padding<=32,'invalid padding');
  valid(raw.origin?.pivot?.x===0.5&&raw.origin?.pivot?.y===0.5,'unsupported origin');
  valid(Array.isArray(raw.pages)&&raw.pages.length>0&&Array.isArray(raw.frames),'missing pages/frames');
  const pages=new Map();
  for(const page of raw.pages){
    valid(Number.isInteger(page.index)&&page.index>=0&&!pages.has(page.index),'duplicate/invalid page index');
    valid(positive(page.width)&&positive(page.height)&&page.width<=4096&&page.height<=4096,'invalid page size');
    valid(typeof page.image==='string'&&/^[a-zA-Z0-9_-]+\.png$/.test(page.image),'invalid page filename');
    pages.set(page.index,page);
  }
  valid(raw.frames.length===raw.frame_count,'frame count mismatch');
  const indices=new Set(),names=new Set();
  for(const frame of raw.frames){
    valid(Number.isInteger(frame.index)&&frame.index>=0&&frame.index<raw.frame_count&&!indices.has(frame.index),'duplicate/missing frame index');
    valid(typeof frame.name==='string'&&!names.has(frame.name),'duplicate/missing frame name');
    const page=pages.get(frame.page),r=frame.rect,p=raw.padding;
    valid(page&&r&&[r.x,r.y,r.w,r.h].every(Number.isInteger),'missing page/rectangle');
    valid(r.x>=p&&r.y>=p&&positive(r.w)&&positive(r.h)&&r.x+r.w+p<=page.width&&r.y+r.h+p<=page.height,'frame outside page');
    valid(frame.source_size?.w===r.w&&frame.source_size?.h===r.h,'trimmed frame');
    valid(frame.pivot?.x===raw.origin.pivot.x&&frame.pivot?.y===raw.origin.pivot.y,'inconsistent pivot');
    valid(Math.abs(frame.duration_ms-1000/raw.fps)<1e-6,'unsupported variable frame duration');
    indices.add(frame.index);names.add(frame.name);
  }
  return {...raw,pages:[...pages.values()].sort((a,b)=>a.index-b.index),frames:[...raw.frames].sort((a,b)=>a.index-b.index)};
}
