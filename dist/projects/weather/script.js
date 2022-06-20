var lat = document.getElementById("latitude");
var lon = document.getElementById("longitude");
if (navigator.geolocation) {
   navigator.geolocation.getCurrentPosition(getPos);
}

function getPos(position) {
   lat.value = position.coords.latitude;
   lon.value = position.coords.longitude;
}


async function GetData() {
    let request = await fetch(`https://api.weather.gov/points/${lat.value},${lon.value}`);
    let data = await request.json();

    let request2 = await fetch(data.properties["forecast"]);
    let data2 = await request2.json();

    document.getElementById("p1").innerHTML = data2.properties.periods[0].detailedForecast;
}
