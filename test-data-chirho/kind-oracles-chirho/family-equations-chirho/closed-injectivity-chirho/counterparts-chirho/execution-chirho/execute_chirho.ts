// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)
import { mkdtemp, readFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
const source_chirho=resolve(process.argv[2]);
const binary_chirho=resolve(process.argv[3]);
const output_chirho=process.argv[4];
const scratch_chirho=await mkdtemp(join(tmpdir(),"haskelujah-closed-execution-chirho-"));
const records_chirho=[];
for(const command_chirho of [["ghc","--numeric-version"],["ghc","-v0","-fforce-recomp","-outputdir",scratch_chirho,"-o",join(scratch_chirho,"reference-chirho"),source_chirho],[join(scratch_chirho,"reference-chirho")],[binary_chirho,"run",source_chirho]]) {
  const start_chirho=Date.now();
  const child_chirho=Bun.spawn(command_chirho,{stdout:"pipe",stderr:"pipe"});
  let timed_out_chirho=false;
  const timer_chirho=setTimeout(()=>{timed_out_chirho=true;child_chirho.kill("SIGKILL");},30000);
  const [exit_chirho,stdout_chirho,stderr_chirho]=await Promise.all([child_chirho.exited,new Response(child_chirho.stdout).text(),new Response(child_chirho.stderr).text()]);
  clearTimeout(timer_chirho);
  records_chirho.push({command_chirho,exit_chirho,stdout_chirho,stderr_chirho,timed_out_chirho,elapsed_ms_chirho:Date.now()-start_chirho});
  if(exit_chirho!==0 || timed_out_chirho) break;
}
const hash_chirho=async(path_chirho:string)=>new Bun.CryptoHasher("sha256").update(await readFile(path_chirho)).digest("hex");
const agrees_chirho=records_chirho.length===4 && records_chirho.every(row_chirho=>row_chirho.exit_chirho===0 && !row_chirho.timed_out_chirho) && records_chirho[2].stdout_chirho===records_chirho[3].stdout_chirho;
const result_chirho={source_chirho,source_sha256_chirho:await hash_chirho(source_chirho),binary_sha256_chirho:await hash_chirho(binary_chirho),records_chirho,agrees_chirho};
await Bun.write(output_chirho,JSON.stringify(result_chirho,null,2)+"\n");
console.log(JSON.stringify(result_chirho));
if(!agrees_chirho)process.exitCode=1;
