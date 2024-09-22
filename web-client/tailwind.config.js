/** @type {import('tailwindcss').Config} */
export default {
    darkMode: ["class"],
    content: ["./index.html", "./src/**/*.{js,ts,jsx,tsx}"],
  theme: {
  	colors: {
  		secondary: '#1B1B1B',
  		secondaryDark: 'rgba(20,20,20,0.5)',
  		accent: '#D5CDEE',
  		primary: '#fff',
  		primaryDark: 'rgba(255,255,255,0.5)',
  		border: 'rgba(255,255,255,0.1)',
  		success: '#55FFAD',
  		warning: '#FFD337',
  		error: '#FF3737',
  		info: '#55C2FF'
  	},
  	fontFamily: {
  		sans: ["Jost", "sans-serif"],
  		serif: ["Instrument Serif", "serif"],
  		mono: ["Darker Grotesque", "monospace"]
  	},
  	extend: {
  		borderRadius: {
  			lg: 'var(--radius)',
  			md: 'calc(var(--radius) - 2px)',
  			sm: 'calc(var(--radius) - 4px)'
  		},
  		colors: {},
  		keyframes: {
  			'accordion-down': {
  				from: {
  					height: '0'
  				},
  				to: {
  					height: 'var(--radix-accordion-content-height)'
  				}
  			},
  			'accordion-up': {
  				from: {
  					height: 'var(--radix-accordion-content-height)'
  				},
  				to: {
  					height: '0'
  				}
  			}
  		},
  		animation: {
  			'accordion-down': 'accordion-down 0.2s ease-out',
  			'accordion-up': 'accordion-up 0.2s ease-out'
  		}
  	}
  },
  plugins: [require("tailwindcss-animate")],
};
