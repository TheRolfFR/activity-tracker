function rgbToHex(r: number, g: number, b: number) {
	return '#' + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
}

function hexToRgb(hex: string) {
	const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
	return result
		? {
				r: parseInt(result[1], 16),
				g: parseInt(result[2], 16),
				b: parseInt(result[3], 16)
		  }
		: null;
}

// returns an array of startColor, colors between according to steps, and endColor
function colorRamp(startColor: string, endColor: string, steps: number) {
	const ramp = [];

	ramp.push(startColor);

	const startColorRgb = hexToRgb(startColor);
	const endColorRgb = hexToRgb(endColor);

    if(startColorRgb === null || endColorRgb === null) throw new Error('Failed to parse colors');

	const rInc = Math.round((endColorRgb.r - startColorRgb.r) / (steps + 1));
	const gInc = Math.round((endColorRgb.g - startColorRgb.g) / (steps + 1));
	const bInc = Math.round((endColorRgb.b - startColorRgb.b) / (steps + 1));

	for (let i = 0; i < steps; i++) {
		startColorRgb.r += rInc;
		startColorRgb.g += gInc;
		startColorRgb.b += bInc;

		ramp.push(rgbToHex(startColorRgb.r, startColorRgb.g, startColorRgb.b));
	}
	ramp.push(endColor);

	return ramp;
}

export { rgbToHex, hexToRgb, colorRamp }