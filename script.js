const fs = require('fs');  
const path = require('path');  
const { spawn } = require('child_process');  
const perf = require('performance-now');  
const { exit } = require('process'); 

async function compressFile(inputPath) {  
    const extension = path.extname(inputPath).toLowerCase();  
  
    let command = '';  
    let args = [];  
    if (extension === '.jpg' || extension === '.jpeg') {  
        command = 'ffmpeg';  
        args = ['-hide_banner', '-i', inputPath, '-qscale:v', '2', inputPath];  
    } else if (extension === '.png') {  
        command = 'pngquant';  
        args = ['--quality=65-80', inputPath, '-o', inputPath];  
    } else if (extension === '.tiff') {  
        command = 'convert';  
        args = [inputPath, '-quality', '80', inputPath];  
    } else {  
        console.error(`Неподдерживаемый тип: ${extension}`);  
        return;  
    }  
  
    const proc = spawn(command, args);  
 
    proc.on('error', (error) => {  
        console.error(`error: ${error.message}`);  
    });  
 
    proc.on('close', (code) => {  
        if (code !== 0) {  
            console.log(`${command} process exited with code ${code}`);  
        } else {  
            console.log(`Сжалось ${inputPath}`);  
        }  
    });  
}  
let limit; 

async function compressDirectory(directoryPath) {  
    const files = await fs.promises.readdir(directoryPath, { withFileTypes: true });  
  
    for (const file of files) {  
        const fullPath = path.join(directoryPath, file.name);  
  
        if (fullPath.includes('/compressed')) {  
            continue;   
        }  
  
        if (file.isDirectory()) {  
            await limit(() => compressDirectory(fullPath));  
        } else {  
            await limit(() => compressFile(fullPath));  
            console.log(`Сжимание файла ${fullPath}`);  
        }  
    }  
}  
const t0 = perf();  
// Загрузка p-limit и выполнение сжатия 
import('p-limit').then(pLimit => { 
    limit = pLimit.default(150); 
    return compressDirectory('/data'); 
}).then(() => { 
    const t1 = perf(); 
    console.log(`Функция выполнялась ${t1 - t0} миллисекунд`); 
    exit(); 
}).catch(err => { 
    console.error(err); 
});
