const os = require('os')
const fs = require('fs');
const path = require('path')
// const http = require('http');
// var options = {
//   hostname: 'localhost',
//   port: 18080,
//   path: '/gen-qrcode',
//   method: 'POST',
//   headers: {
//     'Content-Type': 'application/json',
//     'Content-Length': 0
//   }
// }
// const request = http.request(options, res => {
//   res.on('data', d => {
//     ret = d.toString('ascii'); // Trying to set ret as the response data
//   })
// })

// request.write("")
// request.end()

const args = process.argv.slice(2); // arg start from index 2
if (args.length < 2) {
  process.exit(1)
}
const config = JSON.parse(fs.readFileSync(args[0]));
console.log('config', config)

const generate = require('./pdf');
const dir = args[1]//os.homedir() + '/Downloads/output'
const cuttingImg = path.join(dir, 'template.cutting.png')
const outputDir = path.join(dir, 'output')
console.log('outputDir', outputDir)
// list all files in the directory
fs.readdir(outputDir, (err, files) => {
  if (err) {
    throw err;
  }

  // files object contains all files names
  // log them on console
  const images = files
    .filter(file => {
      if (file.toLowerCase().indexOf('.png') > 0 || file.toLowerCase().indexOf('.jpg') > 0) {
        // console.log(file);
        return true
      }
      return false
    })
    .map(file => path.join(outputDir, file))
  // console.log(images)
  generate({ images, outputDir, cuttingImg, config })
});

