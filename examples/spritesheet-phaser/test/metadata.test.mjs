import assert from 'node:assert/strict';
import { test } from 'node:test';
import { readFile } from 'node:fs/promises';
import {createHash} from 'node:crypto';
import { parseSheet } from '../src/metadata.js';

const original=JSON.parse(await readFile(new URL('../assets/walker-walk/spritesheet.json',import.meta.url),'utf8'));
test('real exports and M004.1 references preserve their recorded hashes',async()=>{
  const provenance=JSON.parse(await readFile(new URL('../assets/provenance.json',import.meta.url),'utf8'));
  for(const [name,expected] of Object.entries(provenance.files)){
    const bytes=await readFile(new URL(`../assets/${name}`,import.meta.url));
    assert.equal(createHash('sha256').update(bytes).digest('hex'),expected,name);
  }
});
test('explicit indices reconstruct shuffled page/frame arrays without mutating the input',()=>{
  const input=structuredClone(original);input.frames.reverse();input.pages.reverse();
  const before=JSON.stringify(input);const sheet=parseSheet(input);
  assert.equal(sheet.frames.length,60);assert.equal(sheet.pages.length,7);
  assert.deepEqual(sheet.frames.map(f=>f.index),Array.from({length:60},(_,i)=>i));
  assert.equal(JSON.stringify(input),before);
});
test('unsupported contracts, missing indices and out-of-bounds frames are refused',()=>{
  for(const mutate of [s=>s.spritesheet_version=2,s=>s.frames.pop(),s=>s.frames[1].index=0,
    s=>s.frames[0].rect.x=-1,s=>s.frames[0].page=999,s=>s.pages[0].image='../outside.png',
    s=>s.frames[0].duration_ms=999,s=>s.trim=true,s=>s.frames[0].pivot.x=0]){
    const invalid=structuredClone(original);mutate(invalid);assert.throws(()=>parseSheet(invalid));
  }
});
