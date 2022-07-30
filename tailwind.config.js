module.exports = {
  purge: {
    enabled: process.env.NODE_ENV === 'production',
    mode: 'all',
    // source_code represents the rust (yew?) source code root
    content: ["./src/**/*.rs", "./index.html", "./tailwind/tailwind.css"]
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
        darkest: '#1e2124',
        darker: '#282b30',
        dark: '#36393e',
        light_dark: '#424549',
        light_blue: '#7289da',
        light_gray: '#99aab5',
        gray: {
          900: '#202225',
          800: '#2f3136',
          700: '#36393f',
          600: '#4f545c',
          400: '#d4d7dc',
          300: '#e3e5e8',
          200: '#ebedef',
          100: '#f2f3f5',
        },
      },
    },
  },
  variants: {
    extend: {
      borderRadius: ['hover'],
      visibility: ['group-hover'],
    },
  },
  plugins: [],
}

