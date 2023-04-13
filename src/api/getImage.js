import axios from "axios";

export async function getImageBase64  (imageUrl, headers){
    try {
        const response = await axios.get(imageUrl, {
            responseType: 'arraybuffer',
            headers: headers
        });

        const imageBuffer = Buffer.from(response.data, 'binary');
        const imageBase64 = imageBuffer.toString('base64');

        return imageBase64;
    } catch (error) {
        console.error(error);
    }
};