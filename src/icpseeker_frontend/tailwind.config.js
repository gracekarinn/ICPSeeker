/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,jsx,ts,tsx,scss}",
  ],
  theme: {
    extend: {
      fontFamily: {
        heading: ['"Plus Jakarta Sans"', 'sans-serif'], 
        body: ['"Syne"', 'sans-serif'], 
      },
      fontSize: {
        h1: ['4rem', { lineHeight: '120%', fontWeight: '700' }],
        h2: ['3rem', { lineHeight: '120%', fontWeight: '700' }],
        h3: ['2rem', { lineHeight: '120%', fontWeight: '700' }],
        h4: ['1.5rem', { lineHeight: '120%', fontWeight: '700' }],
        h5: ['1.25rem', { lineHeight: '120%', fontWeight: '700' }],
        h6: ['1rem', { lineHeight: '120%', fontWeight: '700' }],
        p1: ['1.5rem', { lineHeight: '150%', fontWeight: '400' }],
        p2: ['1.25rem', { lineHeight: '150%', fontWeight: '400' }],
        p3: ['1rem', { lineHeight: '150%', fontWeight: '400' }],
        p4: ['0.875rem', { lineHeight: '150%', fontWeight: '400' }],
        p5: ['0.75rem', { lineHeight: '150%', fontWeight: '400' }],
        p6: ['0.625rem', { lineHeight: '150%', fontWeight: '400' }],
      },
      colors: {
        purple: {
          custom: "#523DC6",
          baselight: "F6F4FF",
          light: "#efe7fd", 
          "light-hover": "#e7dbfb", 
          "light-active": "#cdb4f8", 
          normal: "#5f0ee7",
          "normal-hover": "#560dd0", 
          "normal-active": "#4c0bb9", 
          dark: "#470bad", 
          "dark-hover": "#39088b", 
          "dark-active": "#2b0668", 
          darker: "#210551", 
        },
        orange: {
          light: "#fff0e7", 
          "light-hover": "#ffe8db", 
          "light-active": "#ffd0b4", 
          normal: "#ff660e", 
          "normal-hover": "#e65c0d", 
          "normal-active": "#cc520b", 
          dark: "#bf4d0b", 
          "dark-hover": "#993d08", 
          "dark-active": "#732e06", 
          darker: "#592405",
        },
        blue: {
          light: "#f7f2ff", 
          "light-hover": "#f3ecff", 
          "light-active": "#e7d7fe",
          normal: "#b07ffc", 
          "normal-hover": "#9e72e3", 
          "normal-active": "#8d66ca",
          dark: "#845fbd",
          "dark-hover": "#6a4c97", 
          "dark-active": "#4f3971", 
          darker: "#3e2c58", 
        },
        gradient: {
          "start": "#5f0ee7", // Gradient start (Purple)
          "end": "#350881", // Gradient end (Purple light)
        },
        state: {
          success: "#28a745", // State Success (Green)
          danger: "#dc3545", // State Danger (Red)
        },
        neutral: {
          50: "#fafafa",
          100: "#f5f5f5",
          200: "#e5e5e5",
          300: "#d4d4d4",
          400: "#a3a3a3",
          500: "#737373",
          600: "#525252",
          700: "#404040",
          800: "#262626",
          900: "#171717",
          950: "#0a0a0a",
        },
      },
      backgroundImage: {
        'header-gradient': 'linear-gradient(to right, var(--tw-gradient-stops))',
      },
      gradientColorStops: theme => ({
        'start': theme('colors.purple.normal'),
        'end': theme('colors.blue.normal'),
      }),
    },
  },
  plugins: [],
};
