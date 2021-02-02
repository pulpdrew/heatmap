/**
 * The id of the embedded map element
 */
const mapId = "map";

/**
 * The template URL for the map tile server
 */
const tileUrlTemplate = 'https://tile.thunderforest.com/atlas/{z}/{x}/{y}.png?apikey={YOUR_API_KEY}}';

/**
 * The Attribution text that will be shown on the map
 */
const tileAttribution = 'Maps © <a href="http://www.thunderforest.com">Thunderforest</a>, Data © <a href="http://www.openstreetmap.org/copyright">OpenStreetMap contributors</a>';

/**
 * The lines that are currently plotted on the map
 */
let lines = [];

/**
 * The Leaflet.js Map element
 */
var map = L.map(mapId);

// use the OpenStreetMap tile servers
L.tileLayer(tileUrlTemplate, {
    attribution: tileAttribution,
    maxZoom: 18,
    tileSize: 512,
    zoomOffset: -1,
}).addTo(map);

/**
 * Fit the map to the bounds that cover all of the polylines in `lines`
 */
var fitBounds = function () {
    let bounds = L.latLngBounds(paths.flatMap(pts => pts.map(pt => L.latLng(pt[0], pt[1]))))
    map.fitBounds(bounds);
}

/**
 * Plot all of the paths in data.js on the map
 */
var plot = function () {
    // Update the status
    document.getElementById("status").textContent = `Loading...`

    // Remove anything that is plotted
    lines.forEach(l => l.remove());

    // Plot the lines
    let color = document.getElementById("color-input").value || "#FF0000";
    let opacity = document.getElementById("opacity-input").value / 100 || 0.3;
    lines = paths.map(p => L.polyline(p, { color, opacity }).addTo(map));

    // Update the status
    document.getElementById("status").textContent = `Plotted ${lines.length} runs!`
}

// Plot the points when the page loads and zoom to fit.
plot();
fitBounds();