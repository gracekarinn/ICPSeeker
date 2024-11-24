/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./src/**/*.{js,jsx,ts,tsx,scss}",
  ],
  theme: {
    extend: {
      fontFamily: {
        heading: ['"Plus Jakarta Sans"'], // Hanya Plus Jakarta Sans
        body: ['Syne'], // Hanya Syne
      },
      fontSize: {
        // Headings (Plus Jakarta Sans)
        h1: ['4rem', { lineHeight: '120%', fontWeight: '700' }], // 64px, 76.8px line-height
        h2: ['3rem', { lineHeight: '120%', fontWeight: '700' }], // 48px, 57.6px line-height
        h3: ['2rem', { lineHeight: '120%', fontWeight: '700' }], // 32px, 38.4px line-height
        h4: ['1.5rem', { lineHeight: '120%', fontWeight: '700' }], // 24px, 28.8px line-height
        h5: ['1.25rem', { lineHeight: '120%', fontWeight: '700' }], // 20px, 24px line-height
        h6: ['1rem', { lineHeight: '120%', fontWeight: '700' }], // 16px, 19.2px line-height

        // Body (Syne)
        p1: ['1.5rem', { lineHeight: '150%', fontWeight: '400' }], // 24px, 36px line-height
        p2: ['1.25rem', { lineHeight: '150%', fontWeight: '400' }], // 20px, 30px line-height
        p3: ['1rem', { lineHeight: '150%', fontWeight: '400' }], // 16px, 24px line-height
        p4: ['0.875rem', { lineHeight: '150%', fontWeight: '400' }], // 14px, 21px line-height
        p5: ['0.75rem', { lineHeight: '150%', fontWeight: '400' }], // 12px, 18px line-height
        p6: ['0.625rem', { lineHeight: '150%', fontWeight: '400' }], // 10px, 15px line-height
      },
    },
  },
  plugins: [],
};
