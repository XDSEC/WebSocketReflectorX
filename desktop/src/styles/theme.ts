export interface Theme {
  primary: string
  secondary: string
  accent: string
  neutral: string
  'base-100': string
  'base-content': string
  info: string
  success: string
  warning: string
  error: string
}

export const themeKeys = [
  // 'ayaka',
  'cyber'
] as const
export type ThemeKey = typeof themeKeys[number]

export const themes: Record<string, Theme> = {
  'cyber-dark': {
    'primary': '#3399FF',
    'secondary': '#60a5fa',
    'accent': '#1FB2A6',
    'neutral': '#202020',
    'base-100': '#121212',
    'base-content': '#D0D0D0',
    'info': '#3399FF',
    'success': '#36D399',
    'warning': '#FBBD23',
    'error': '#F83030'
  },
  'cyber-light': {
    'primary': '#0078D6',
    'secondary': '#60a5fa',
    'accent': '#1FB2A6',
    'neutral': '#F0F0F0',
    'base-100': '#FFFFFF',
    'base-content': '#333333',
    'info': '#0078D6',
    'success': '#36AA3A',
    'warning': '#ca9f00',
    'error': '#F83030'
  }
}

export function getTheme(theme: string) {
  return themes[theme]
}
