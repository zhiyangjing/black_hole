import init, {
    initialize,
    load_texture,
    render
} from '../pkg/black_hole_wasm.js';

let _initialized = false;

/**
 * 初始化 WASM 模块，并在贴图加载后启动渲染循环
 * @param canvasId 页面上 <canvas id="...">
 * @param imgSrc   图片路径
 */
export async function setupWasm(canvasId: string, imgSrc: string) {
    if (!_initialized) {
        await init();
        initialize();
        _initialized = true;
    }

    // 获取 canvas 元素
    const canvas = document.getElementById(canvasId) as HTMLCanvasElement;
    if (!canvas) {
        console.error(`Canvas with id "${canvasId}" not found`);
        return;
    }

    // 加载图片并传给 wasm
    const img = new Image();
    img.crossOrigin = 'anonymous';
    img.src = imgSrc;
    img.onload = () => {
        load_texture(img);
        // 贴图加载完成后才启动渲染循环
        function loop() {
            const ctx = canvas.getContext('2d');
            if (ctx) {
                render(ctx);
            }
            requestAnimationFrame(loop);
        }
        requestAnimationFrame(loop);
    };
}
