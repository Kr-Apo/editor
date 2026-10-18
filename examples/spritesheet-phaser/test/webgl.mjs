import assert from 'node:assert/strict';
import {mkdir,writeFile} from 'node:fs/promises';
import {resolve} from 'node:path';
import {chromium} from 'playwright';
import {start} from '../server.mjs';

const proofs=resolve(process.env.ANKHIMATE_PROOFS??'proofs');await mkdir(proofs,{recursive:true});
const server=await start(0);let browser;
const errors=[],warnings=[],failedRequests=[],comparisons=[],timings=[];
try{
  browser=await chromium.launch({channel:'chromium',...(process.env.ANKHIMATE_CHROMIUM?{executablePath:process.env.ANKHIMATE_CHROMIUM}:{}),headless:true,args:['--disable-background-timer-throttling','--disable-renderer-backgrounding']});
  const page=await browser.newPage({viewport:{width:1200,height:1000},deviceScaleFactor:1});
  page.on('pageerror',error=>errors.push(String(error)));
  page.on('console',message=>{if(message.type()==='error')errors.push(message.text());if(message.type()==='warning')warnings.push(message.text());});
  page.on('requestfailed',request=>failedRequests.push(request.url()));
  page.on('response',response=>{if(response.status()>=400)failedRequests.push(`${response.status()} ${response.url()}`);});
  await page.goto(server.url);
  await page.waitForFunction(()=>window.demoReady||window.demoError,null,{timeout:30000});
  assert.equal(await page.evaluate(()=>window.demoError),undefined);
  const gpu=await page.evaluate(()=>{const gl=window.sheetDemo.game.renderer.gl,ext=gl.getExtension('WEBGL_debug_renderer_info');return ext?gl.getParameter(ext.UNMASKED_RENDERER_WEBGL):gl.getParameter(gl.RENDERER);});
  const clips=await page.evaluate(()=>window.sheetDemo.clips);
  for(const clip of clips){
    await page.evaluate(id=>window.sheetDemo.select(id),clip);
    await page.waitForFunction(()=>window.sheetDemo.state().loops>=3,null,{timeout:10000});
    const state=await page.evaluate(()=>window.sheetDemo.state());
    assert.equal(state.webgl,true);assert.equal(state.phaser,'3.90.0');assert.equal(state.fps,60);assert.equal(state.duration,1000);assert.equal(state.pages,7);
    assert.equal(state.seen.length,60,'all frame changes are observed');assert.deepEqual(state.origin,[0.5,0.5]);assert.equal(state.x,550);assert.equal(state.y,300);
    const intervals=state.repeatTimes.slice(1).map((time,i)=>time-state.repeatTimes[i]);
    assert(intervals.every(ms=>Math.abs(ms-1000)<=200),`loop speed ${intervals.join(', ')} ms`);
    timings.push({clip,loops:state.loops,frameChanges:state.updates,seenFrames:state.seen.length,fps:state.fps,animationDurationMs:state.duration,loopIntervalsMs:intervals});
    for(const index of [0,15,30,45]){
      const result=await page.evaluate(index=>window.sheetDemo.compare(index),index);comparisons.push(result);
      assert.equal(result.badPixels,0,`${clip} frame ${index}: spritesheet / M004.1 GPU pixel mismatch >2/255 (max ${result.maxError})`);
      assert(result.transparent>0&&result.visible>0,'RGBA transparency and visible art');
      assert.equal(result.x,275);assert.equal(result.y,300);assert.deepEqual(result.origin,[0.5,0.5]);
      if(index===15)await page.screenshot({path:resolve(proofs,`${clip}.png`),fullPage:true});
    }
  }
  assert.deepEqual(errors,[]);assert.deepEqual(warnings,[]);assert.deepEqual(failedRequests,[]);
  const result={passed:true,phaser:'3.90.0',browser:browser.version(),gpu,timings,comparisons,errors,warnings,failedRequests};
  await writeFile(resolve(proofs,'webgl-results.json'),JSON.stringify(result,null,2));
  console.log(JSON.stringify(result,null,2));
}finally{await browser?.close();await server.close();}
