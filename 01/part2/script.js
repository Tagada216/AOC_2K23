const fs = require('fs');
const readline = require('readline');

// async function processFile(filePath) {
//     const fileStream = fs.createReadStream(filePath);

//     const rl = readline.createInterface({
//         input: fileStream,
//         crlfDelay: Infinity
//     });

//     let totalSum = 0;



//     for await (const line of rl) {

      
//         let firstDigit = null;
//         let lastDigit = null;

//         for (const char of line) {
//             if (!isNaN(char) && char.trim() !== '') {
//                 const digit = parseInt(char, 10);
//                 if (firstDigit === null) {
//                     firstDigit = digit;
//                 }
//                 lastDigit = digit;
//             }
//         }

//         let sum = 0;
//         if (firstDigit !== null && lastDigit !== null) {
//             sum = (firstDigit === lastDigit) ? firstDigit * 11 : 10 * firstDigit + lastDigit;
//         }
        
//         totalSum += sum;
//         console.log("Line",":",line, "Sum",sum, ":",totalSum);
//     }

//     console.log(`Somme totale: ${totalSum}`);
// }


// processFile('outputTest.txt');


async function processFile(inputFilePath, outputFilePath) {
    const fileStream = fs.createReadStream(inputFilePath);
    const outputFileStream = fs.createWriteStream(outputFilePath);

    const rl = readline.createInterface({
        input: fileStream,
        crlfDelay: Infinity
    });

    const wordToDigit = {
        one: 1,
        two: 2,
        three: 3,
        four: 4,
        five: 5,
        six: 6,
        seven: 7,
        eight: 8,
        nine: 9
    };

    for await (const line of rl) {
        let modifiedLine = line;
        for (const word in wordToDigit) {
            console.log('line: ', line,"word: ", word);
            const regex = new RegExp(word, 'g');
            modifiedLine = modifiedLine.replace(regex, wordToDigit[word]);
        }
        outputFileStream.write(modifiedLine + '\n');
    }

    outputFileStream.end();
}

processFile('test.txt', 'outputTest.txt');