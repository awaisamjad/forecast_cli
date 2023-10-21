# Tasks
1. ~~Ask if user has account or not~~
    1. ~~If they do, ask for api key~~
    2. ~~If they dont, provide url ~~
2. Save api key so they dont have to add it again
   1. This could be done by having a empty path variable
   2. then update it when the user adds that path
   3. then make program use that path
   4. If the path is empty, ask for api key and if it exists dont
3. Ask user to add city name
4. Get weather data from api
5. Display weather data
6. Repeat 3-5

# Extra
1. Add a way for the user to change the api key
2. maybe a way for them to create an accpunt via the cli



- First we check if the api key exists by checking for the path
- If it does, we use it
- If it doesnt, we ask for it and save it
- Then we update that path variable with the one given by user




# Important fields
- Weather
  - main
  - description
- Main
  - temp
  - feels_like
  - temp_min
  - temp_max
  - pressure
  - humidity
- visibility
- wind
  - speed
  - deg
- clouds
- Sys
  - country
  - sunrise
  - sunset
- timezone
- name 

output:
 ```
WeatherData {
    coord: Coord {
        lon: -0.1257,
        lat: 51.5085,
    },
    weather: [
        Weather {
            id: 804,
            main: "Clouds",
            description: "overcast clouds",
            icon: "04n",
        },
    ],
    base: "stations",
    main: Main {
        temp: 283.64,
        feels_like: 283.2,
        temp_min: 282.25,
        temp_max: 285.01,
        pressure: 991,
        humidity: 94,
    },
    visibility: 10000,
    wind: Wind {
        speed: 5.14,
        deg: 290,
    },
    clouds: Clouds {
        all: 100,
    },
    dt: 1697916440,
    sys: Sys {
        sys_type: 2,
        id: 2006068,
        country: "GB",
        sunrise: 1697870029,
        sunset: 1697907375,
    },
    timezone: 3600,
    id: 2643743,
    name: "London",
    cod: 200,
}
```