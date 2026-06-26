/** @type {import('tailwindcss').Config} */
export default {
    darkMode: ["class"],
    content: [
        "./src/**/*.{ts,tsx}",
    ],
    theme: {
        container: {
            center: true,
            padding: "2rem",
            screens: {
                "2xl": "1400px",
            },
        },
        extend: {
            colors: {
                border: "hsl(var(--border))",
                input: "hsl(var(--input))",
                ring: "hsl(var(--ring))",
                background: "hsl(var(--background))",
                foreground: "hsl(var(--foreground))",
                primary: {
                    DEFAULT: "hsl(var(--primary))",
                    foreground: "hsl(var(--primary-foreground))",
                },
                secondary: {
                    DEFAULT: "hsl(var(--secondary))",
                    foreground: "hsl(var(--secondary-foreground))",
                },
                muted: {
                    DEFAULT: "hsl(var(--muted))",
                    foreground: "hsl(var(--muted-foreground))",
                },
                accent: {
                    DEFAULT: "hsl(var(--accent))",
                    foreground: "hsl(var(--accent-foreground))",
                },
                destructive: {
                    DEFAULT: "hsl(var(--destructive))",
                    foreground: "hsl(var(--destructive-foreground))",
                },
                card: {
                    DEFAULT: "hsl(var(--card))",
                    foreground: "hsl(var(--card-foreground))",
                },
                popover: {
                    DEFAULT: "hsl(var(--popover))",
                    foreground: "hsl(var(--popover-foreground))",
                },
            },
            borderRadius: {
                lg: "var(--radius)",
                md: "calc(var(--radius) - 2px)",
                sm: "calc(var(--radius) - 4px)",
            },
            boxShadow: {
                sm: "0 1px 2px 0 rgb(0 0 0 / 0.05)",
                DEFAULT: "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
                md: "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
                lg: "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
                xl: "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
            },
            keyframes: {
                "pulse-border": {
                    "0%, 100%": {
                        borderColor: "hsl(var(--border))",
                    },
                    "50%": {
                        borderColor: "hsl(var(--primary))",
                    },
                },
                "flow-line": {
                    "0%": {
                        backgroundPosition: "0% 0%",
                    },
                    "100%": {
                        backgroundPosition: "100% 100%",
                    },
                },
                "accordion-down": {
                    from: { height: 0 },
                    to: { height: "var(--radix-accordion-content-height)" },
                },
                "accordion-up": {
                    from: { height: "var(--radix-accordion-content-height)" },
                    to: { height: 0 },
                },
                "flow-edge": {
                    from: { strokeDashoffset: "0" },
                    to: { strokeDashoffset: "-24" },
                },
                "flow-edge-running": {
                    from: { strokeDashoffset: "0" },
                    to: { strokeDashoffset: "-28" },
                },
            },
            animation: {
                "pulse-border": "pulse-border 2s ease-in-out infinite",
                "flow-line": "flow-line 2s linear infinite",
                "accordion-down": "accordion-down 0.2s ease-out",
                "accordion-up": "accordion-up 0.2s ease-out",
                "flow-edge": "flow-edge 1.5s linear infinite",
                "flow-edge-running": "flow-edge-running 0.8s linear infinite",
            },
        },
    },
    plugins: [require("tailwindcss-animate")],
};