/** @type {import('tailwindcss').Config} */
const plugin = require('tailwindcss/plugin')
const defaultTheme = require('tailwindcss/defaultTheme')
const colors = require('tailwindcss/colors');

const cartesian = (...a) => a.reduce((a, b) => a.flatMap(d => b.map(e => [d, e].flat())));

module.exports = {
    content: {
        files: ['*.html', './web/app/**/*.rs', './web/front-csr/**/*.rs', '../app/**/*.rs', 'src/**/*.rs'],
    },
    safelist: cartesian(
        [
            'border-cc',
            'border-r-cc',
            'focus-visible:border-cc',
            'bg-cc',
        ],
        [
            'death-knight',
            'demon-hunter',
            'druid',
            'evoker',
            'hunter',
            'mage',
            'monk',
            'paladin',
            'priest',
            'rogue',
            'shaman',
            'warlock',
            'warrior',
            'general'
        ]
    ).map(([cssClass, gameClass]) => cssClass + '-' + gameClass),
    theme: {
        fontFamily: {
            title: ['Comfortaa'],
        },
        extend: {
            keyframes: {
                expand: {
                    '0%': {width: 0},
                    '100%': {},
                }
            },
            animation: {
                expand: 'expand 1s ease-in-out',
            },
            colors: {
                cc: {
                    'death-knight': '#C41E3A',
                    'demon-hunter': '#A330C9',
                    'druid': '#FF7C0A',
                    'evoker': '#33937F',
                    'hunter': '#AAD372',
                    'mage': '#3FC7EB',
                    'monk': '#00FF98',
                    'paladin': '#F48CBA',
                    'priest': '#FFFFFF',
                    'rogue': '#FFF468',
                    'shaman': '#0070DD',
                    'warlock': '#8788EE',
                    'warrior': '#C69B6D',
                    //'general': colors.slate['900'],
                    'general': '#00000000',
                }
            },
            blur: {
                xs: '1px',
            },
            contrast: {
                300: '300%',
            },
            textShadow: {
                sm: '0 1px 2px var(--tw-shadow-color)',
                DEFAULT: '0 2px 4px var(--tw-shadow-color)',
                lg: '0 8px 16px var(--tw-shadow-color)',
                outline: '-1px -1px 0 var(--tw-shadow-color), 1px -1px 0 var(--tw-shadow-color), -1px 1px 0 var(--tw-shadow-color), 1px 1px 0 var(--tw-shadow-color)',
            },
            borderWidth: {
                '3': '3px',
            }
        },
    },
    plugins: [
        plugin(function ({matchUtilities, theme}) {
            matchUtilities(
                {
                    'text-shadow': (value) => ({
                        textShadow: value,
                    }),
                },
                {values: theme('textShadow')}
            )
        }),
        require('tailwindcss-animate'),
        //require('@tailwindcss/forms'),
        require('@tailwindcss/container-queries'),
    ],
}
