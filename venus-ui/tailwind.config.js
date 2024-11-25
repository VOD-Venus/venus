/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: ["class", '[data-theme="dark"]'],
    content: {
        files: ["*.html", "./src/**/*.rs", "./public/**/*.svg"],
    },
    theme: {
        extend: {
            keyframes: {
                "slide-in-right": {
                    "0%": {
                        transform: "translateX(100%)",
                    },
                    "100%": {
                        transform: "translateX(0)",
                    },
                },
                "slide-out-right": {
                    "0%": {
                        transform: "translateX(0)",
                    },
                    "100%": {
                        transform: "translateX(100%)",
                    },
                },
            },
            animation: {
                "slide-in-right": "slide-in-right 0.3s ease-out",
                "slide-in-right-slow": "slide-in-right 1s ease-out",
                "slide-out-right": "slide-out-right 0.3s ease-out",
                "slide-out-right-slow": "slide-out-right 1s ease-out",
            },
            colors: {
                bluish: {
                    gray: "rgba(245,247,250)",
                },
                rua: {
                    gray: {
                        100: "#aabfc5",
                        600: "rgb(66,66,66)",
                        700: "hsl(220, 13%, 18%)", // code background in dark
                        800: "rgb(35,38,38)", // card background in dark
                        // 900: "rgb(24,25,26)", // body background in dark
                        900: "#181e24",
                    },
                },
            },
        },
    },
    plugins: [require("daisyui")],
    daisyui: {
        themes: [
            "light",
            "dark",
            // "cupcake",
            // "bumblebee",
            // "emerald",
            // "corporate",
            // "synthwave",
            // "retro",
            // "cyberpunk",
            // "valentine",
            // "halloween",
            // "garden",
            // "forest",
            // "aqua",
            // "lofi",
            // "pastel",
            // "fantasy",
            // "wireframe",
            // "black",
            // "luxury",
            // "dracula",
            // "cmyk",
            // "autumn",
            // "business",
            // "acid",
            // "lemonade",
            // "night",
            // "coffee",
            // "winter",
            // "dim",
            // "nord",
            // "sunset",
        ],
    },
};
