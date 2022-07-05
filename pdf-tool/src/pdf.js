const PDFDocument = require('pdfkit');
const fs = require('fs');
const path = require('path')

module.exports = function generate({ images, outputDir, cuttingImg, config } = { images: [] }) {
  const {
    size = 1000,
    width: _width, height: _height,
    page_w, page_h,
    padding_x: paddingX = 0, padding_y: paddingY = 0,
    page_num_x = 5, page_num_y = 5, pageNumFontsize = 8, pageNumFont = './simhei.ttf',
    margin_x: mx, margin_y: my, page_size: pageSize = 36, cols
  } = config.pdf
  const ratio = 72 / 300 // 72 dpi to 300
  const width = _width * ratio, height = _height * ratio
  const pageW = page_w * ratio, pageH = page_h * ratio
  const marginX = mx * ratio, marginY = my * ratio
  const pageNumX = page_num_x * ratio, pageNumY = page_num_y * ratio

  function _func(images, pdfNum) {
    // Create a document
    // The size property can be either an array specifying [width, height] in PDF points (72 per inch), 
    // or a string specifying a predefined size. 
    const doc = new PDFDocument({
      size: [pageW, pageH],//https://pdfkit.org/docs/paper_sizes.html
      margins: {
        top: 0,
        bottom: 0,
        left: 0,
        right: 0
      }
    });
    doc.pipe(fs.createWriteStream(path.join(outputDir, `output-${pdfNum}.pdf`)));

    // Embed a font, set the font size, and render some text
    // doc
    //   .font('fonts/PalatinoBold.ttf')
    //   .fontSize(25)
    //   .text('Some text with an embedded font!', 100, 100);

    // Add an image, constrain it to a given size, and center it vertically and horizontally

    images.forEach((img, i) => {

      const page = i % pageSize

      if (page === 0 && i > 0) {
        console.log('new pdf page', i)
        // Add another page
        doc.addPage()
      }

      const row = Math.floor(page / cols)
      const col = page % cols
      // console.log(row, col, img)
      var _ = doc.image(img, col * (width + marginX) + paddingX, row * (height + marginY) + paddingY, {
        width, height,
        // fit: [150, 200],
        // align: 'center',
        // valign: 'center' 
      });

      // keep at the top
      if (page === 0) {
        const pageNum = Math.floor(i / pageSize) + 1
        console.log('pageNum', pageNum)
        doc.fontSize(pageNumFontsize);
        doc
        // .font(pageNumFont)
        .text(`页码:${pad(pageNum, 3)}`, pageNumX, pageNumY, {
          // width: pageNumW, height: pageNumH,
          align: 'left'
        });
      }
    })

    // Finalize PDF file
    doc.end();
  }
  let pdfNum = 0
  while (images.length > 0) {
    _func(images.splice(0, size), ++pdfNum)
  }




  // For cutting
  const cutting = new PDFDocument({
    size: [pageW, pageH],//https://pdfkit.org/docs/paper_sizes.html
    margins: {
      top: 0,
      bottom: 0,
      left: 0,
      right: 0
    }
  });
  cutting.pipe(fs.createWriteStream(path.join(outputDir, 'output.cutting.pdf')));
  console.log(cuttingImg)
  for (var i = 0; i < pageSize; i++) {
    const row = Math.floor(i / cols)
    const col = i % cols
    var _ = cutting.image(cuttingImg, col * (width + marginX), row * (height + marginY), {
      width, height,
    });
  }
  cutting.end();
}

function pad(number, length) {
  var str = '' + number;
  while (str.length < length) {
    str = '0' + str;
  }
  return str;

}