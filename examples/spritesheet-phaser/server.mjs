import { createServer } from 'node:http';
import { readFile,realpath,stat } from 'node:fs/promises';
import { dirname,extname,isAbsolute,relative,resolve,sep } from 'node:path';
import { fileURLToPath } from 'node:url';

export async function start(port=4174){
  const root=await realpath(dirname(fileURLToPath(import.meta.url)));
  const mime={'.html':'text/html','.js':'text/javascript','.json':'application/json','.png':'image/png','.css':'text/css'};
  const server=createServer(async(req,res)=>{
    try{
      const url=new URL(req.url,'http://localhost');
      const pathname=decodeURIComponent(url.pathname==='/'?'/index.html':url.pathname);
      if(pathname.split('/').some(p=>p.startsWith('.')))throw new Error('private path');
      const path=await realpath(resolve(root,`.${pathname}`)),local=relative(root,path);
      if(isAbsolute(local)||local==='..'||local.startsWith(`..${sep}`)||!(await stat(path)).isFile())throw new Error('outside root');
      res.writeHead(200,{'Content-Type':mime[extname(path)]??'application/octet-stream','Cache-Control':'no-store'});res.end(await readFile(path));
    }catch{res.writeHead(404);res.end('Not found');}
  });
  await new Promise((done,reject)=>{server.once('error',reject);server.listen(port,'127.0.0.1',done);});
  return {url:`http://127.0.0.1:${server.address().port}`,close:()=>new Promise((done,reject)=>server.close(e=>e?reject(e):done()))};
}
if(process.argv[1]&&resolve(process.argv[1])===fileURLToPath(import.meta.url)){
  const server=await start(Number(process.env.ANKHIMATE_PORT??4174));console.log(`Ankhimate spritesheets: ${server.url}`);
}
