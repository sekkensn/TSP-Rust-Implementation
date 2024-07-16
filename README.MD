# Traveling Salesman Problem implementation in Rust

A Rust-based project that reads a list of cities from a file, allows the user to select a starting city, and calculates the shortest path through a specified number of cities using a simple nearest-neighbor algorithm.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [License](#license)

## Introduction

The Image Randomizer project loads a list of cities with coordinates from a file, prompts the user to select a starting city, and calculates the shortest path through a specified number of cities using a nearest-neighbor algorithm. The result includes the order of cities visited and the total distance traveled.

## Features

- Load a list of cities from a file.
- Prompt the user to select a starting city.
- Calculate the shortest path through a specified number of cities using a nearest-neighbor algorithm.
- Display the order of cities visited and the total distance traveled.

## Installation

To install and run this project, you need to have Rust installed on your machine. Follow the steps below to set up the project:

1. Clone the repository:

    ```bash
    git clone https://github.com/sekkensn/Image-Randomizer.git
    cd Image-Randomizer
    ```

2. Build the project:

    ```bash
    cargo build
    ```

3. Run the project:

    ```bash
    cargo run
    ```

## Usage

When you run the project, it will:

1. Load a list of cities from `src/default_cities.txt`.
2. Prompt you to enter the starting city from the loaded cities.
3. Prompt you to enter the number of cities to include in the path.
4. Display the shortest path from the starting city through the specified number of cities and the total distance traveled.

The expected format for the `default_cities.txt` file is:

```text
CityName, x_coordinate, y_coordinate
CityName2, x_coordinate2, y_coordinate2
```


### Example

1. The program loads the cities from `src/default_cities.txt`.
2. Prompts the user:

    ```text
    Enter the starting city:
    ```

3. Prompts the user to enter the number of cities:

    ```text
    Enter the number of cities:
    ```

4. Displays the shortest path and total distance:

    ```text
    Shortest path from starting city:
    From City Vilnius to City Berlin - Distance 1207 km
    From City Berlin to City Rome - Distance 1066 km
    ...
    Total Distance: 12345 km
    ```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.
"# TSP-Rust-Implementation" 
