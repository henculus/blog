import Quill from 'quill'
let EmbedBlot = Quill.import('blots/embed')

function warnAboutOptions(options) {
    // Safe-ify Options
    options.maxWidth = options.maxWidth || 1000;
    options.maxHeight = options.maxHeight || 1000;

    if (options.maxWidth && typeof options.maxWidth !== "number") {
        console.warn(
            `[config error] 'maxWidth' is required to be a "number" (in pixels), 
recieved: ${options.maxWidth}
-> using default 1000`
        );
        options.maxWidth = 1000;
    }
    if (options.maxHeight && typeof options.maxHeight !== "number") {
        console.warn(
            `[config error] 'maxHeight' is required to be a "number" (in pixels), 
recieved: ${options.maxHeight}
-> using default 1000`
        );
        options.maxHeight = 1000;
    }
    if (options.quality && typeof options.quality !== "number") {
        console.warn(
            `quill.imageCompressor: [config error] 'quality' is required to be a "number", 
recieved: ${options.quality}
-> using default 0.7`
        );
        options.quality = 0.7;
    }
    if (
        options.imageType &&
        (typeof options.imageType !== "string" ||
            !options.imageType.startsWith("image/"))
    ) {
        console.warn(
            `quill.imageCompressor: [config error] 'imageType' is required be in the form of "image/png" or "image/jpeg" etc ..., 
recieved: ${options.imageType}
-> using default image/jpeg`
        );
        options.imageType = "image/jpeg";
    }
}

class imageCompressor{
    constructor(quill, options) {
        this.quill = quill;
        this.range = null;
        this.options = options;
        this.debug = options.debug == null || options.debug == true;

        warnAboutOptions(options);

        var toolbar = this.quill.getModule("toolbar");
        toolbar.addHandler("image", this.selectLocalImage.bind(this));
    }

    selectLocalImage() {
        this.range = this.quill.getSelection();
        this.fileHolder = document.createElement("input");
        this.fileHolder.setAttribute("type", "file");
        this.fileHolder.setAttribute("accept", "image/*");
        this.fileHolder.setAttribute("style", "visibility:hidden");

        this.fileHolder.onchange = this.fileChanged.bind(this);

        document.body.appendChild(this.fileHolder);

        this.fileHolder.click();

        window.requestAnimationFrame(() => {
            document.body.removeChild(this.fileHolder);
        });
    }

    fileChanged() {
        const file = this.fileHolder.files[0];
        if (!file) {
            return;
        }

        const fileReader = new FileReader();

        fileReader.addEventListener(
            "load",
            async () => {
                const base64ImageSrc = fileReader.result;
                const base64ImageSrcNew = await downscaleImage(
                    base64ImageSrc,
                    this.options.maxWidth,
                    this.options.maxHeight,
                    this.options.imageType,
                    this.options.quality,
                    this.debug
                );
                this.insertToEditor(base64ImageSrcNew);
            },
            false
        );
        fileReader.readAsDataURL(file);
    }

    insertToEditor(url) {
        const range = this.range;
        // Insert the compressed image
        this.logFileSize(url);
        this.quill.insertEmbed(range.index, "lazyImage", `${url}`);
        // Move cursor to next position
        range.index++;
        this.quill.setSelection(range, "api");
    }

    logFileSize(dataUrl) {
        const head = "data:image/png;base64,";
        const fileSizeBytes = Math.round(((dataUrl.length - head.length) * 3) / 4);
        const fileSizeKiloBytes = (fileSizeBytes / 1024).toFixed(0);
        if (this.debug) {
            console.log(
                "quill.imageCompressor: estimated img size: " +
                fileSizeKiloBytes +
                " kb"
            );
        }
    }
}

// Take an image URL, downscale it to the given width, and return a new image URL.
async function downscaleImage(
    dataUrl,
    maxWidth,
    maxHeight,
    imageType,
    imageQuality,
    debug
) {
    "use strict";
    // Provide default values
    imageType = imageType || "image/jpeg";
    imageQuality = imageQuality || 0.7;

    // Create a temporary image so that we can compute the height of the downscaled image.
    const image = new Image();
    image.src = dataUrl;
    await new Promise(resolve => {
        image.onload = () => {
            resolve();
        };
    });
    const [newWidth, newHeight] = getDimensions(
        image.width,
        image.height,
        maxWidth,
        maxHeight
    );

    // Create a temporary canvas to draw the downscaled image on.
    const canvas = document.createElement("canvas");
    canvas.width = newWidth;
    canvas.height = newHeight;

    // Draw the downscaled image on the canvas and return the new data URL.
    const ctx = canvas.getContext("2d");
    ctx.drawImage(image, 0, 0, newWidth, newHeight);
    const newDataUrl = canvas.toDataURL(imageType, imageQuality);
    if (debug) {
        console.log("quill.imageCompressor: downscaling image...", {
            args: {
                dataUrl,
                newWidth,
                imageType,
                imageQuality
            },
            image,

            newHeight,
            canvas,
            ctx,
            newDataUrl
        });
    }
    return newDataUrl;
}

function getDimensions(inputWidth, inputHeight, maxWidth, maxHeight) {
    if (inputWidth < maxWidth && inputHeight < maxHeight) {
        return [inputWidth, inputHeight];
    }
    if (inputWidth > maxWidth) {
        const newWidth = maxWidth;
        const newHeight = Math.floor((inputHeight / inputWidth) * newWidth);

        if (newHeight > maxHeight) {
            const newHeight = maxHeight;
            const newWidth = Math.floor((inputWidth / inputHeight) * newHeight);
            return [newWidth, newHeight];
        }
        else {
            return [newWidth, newHeight];
        }
    }
    if (inputHeight > maxHeight) {
        const newHeight = maxHeight;
        const newWidth = Math.floor((inputWidth / inputHeight) * newHeight);
        return [newWidth, newHeight];
    }
}

class LazyImage extends EmbedBlot {
    static create(url) {
        const node = super.create(url)
        node.setAttribute('src', url)
        return node
    }
}
LazyImage.blotName = 'lazyImage'
LazyImage.tagName = 'img'

window.imageCompressor = imageCompressor;
export { imageCompressor };
export default imageCompressor;
export { LazyImage }
