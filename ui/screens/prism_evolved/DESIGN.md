---
name: Prism Evolved
colors:
  surface: '#11131b'
  surface-dim: '#11131b'
  surface-bright: '#373942'
  surface-container-lowest: '#0c0e16'
  surface-container-low: '#191b23'
  surface-container: '#1d1f28'
  surface-container-high: '#282a32'
  surface-container-highest: '#33343d'
  on-surface: '#e1e1ed'
  on-surface-variant: '#c8c4d7'
  inverse-surface: '#e1e1ed'
  inverse-on-surface: '#2e3039'
  outline: '#928ea0'
  outline-variant: '#474554'
  surface-tint: '#c6bfff'
  primary: '#c6bfff'
  on-primary: '#2900a0'
  primary-container: '#6c5ce7'
  on-primary-container: '#faf6ff'
  inverse-primary: '#5847d2'
  secondary: '#c6bfff'
  on-secondary: '#28069c'
  secondary-container: '#4331b4'
  on-secondary-container: '#b7afff'
  tertiary: '#ffb77d'
  on-tertiary: '#4d2600'
  tertiary-container: '#ac5d00'
  on-tertiary-container: '#fff5f1'
  error: '#ffb4ab'
  on-error: '#690005'
  error-container: '#93000a'
  on-error-container: '#ffdad6'
  primary-fixed: '#e4dfff'
  primary-fixed-dim: '#c6bfff'
  on-primary-fixed: '#160066'
  on-primary-fixed-variant: '#4029ba'
  secondary-fixed: '#e4dfff'
  secondary-fixed-dim: '#c6bfff'
  on-secondary-fixed: '#160066'
  on-secondary-fixed-variant: '#402eb1'
  tertiary-fixed: '#ffdcc3'
  tertiary-fixed-dim: '#ffb77d'
  on-tertiary-fixed: '#2f1500'
  on-tertiary-fixed-variant: '#6e3900'
  background: '#11131b'
  on-background: '#e1e1ed'
  surface-variant: '#33343d'
  surface-card: '#1B1E29'
  surface-elevated: '#20232F'
  text-primary: '#FFFFFF'
  text-secondary: '#8A8F9C'
  status-success: '#4ADE80'
  status-error: '#EF4444'
  status-warning: '#F59E0B'
typography:
  headline-lg:
    fontFamily: Inter
    fontSize: 24px
    fontWeight: '700'
    lineHeight: 32px
    letterSpacing: -0.02em
  headline-md:
    fontFamily: Inter
    fontSize: 18px
    fontWeight: '600'
    lineHeight: 24px
  body-lg:
    fontFamily: Inter
    fontSize: 16px
    fontWeight: '400'
    lineHeight: 24px
  body-md:
    fontFamily: Inter
    fontSize: 14px
    fontWeight: '400'
    lineHeight: 20px
  body-sm:
    fontFamily: Inter
    fontSize: 13px
    fontWeight: '400'
    lineHeight: 18px
  label-md:
    fontFamily: Inter
    fontSize: 12px
    fontWeight: '500'
    lineHeight: 16px
    letterSpacing: 0.01em
  label-sm:
    fontFamily: Inter
    fontSize: 11px
    fontWeight: '600'
    lineHeight: 14px
rounded:
  sm: 0.25rem
  DEFAULT: 0.5rem
  md: 0.75rem
  lg: 1rem
  xl: 1.5rem
  full: 9999px
spacing:
  container-margin: 24px
  gutter: 16px
  card-padding: 16px
  list-gap: 8px
  sidebar-width: 240px
  detail-panel-width: 320px
---

## Brand & Style

The design system focuses on a **Professional Gaming / SaaS Hybrid** aesthetic. It targets power users and enthusiasts who require a functional, high-density tool that feels modern and premium. 

The personality is **Focused, Technical, and Reliable**. The visual direction uses a **Corporate Modern** foundation mixed with **Glassmorphism** for depth. The interface prioritizes clear information architecture and rapid action through a sophisticated dark palette and high-contrast accent points. It evokes a sense of "readiness to play" without sacrificing the utility of a complex management tool.

## Colors

The palette is anchored in deep graphite and navy tones to reduce eye strain during long gaming sessions. 

- **Primary & Secondary:** A vibrant purple-to-indigo gradient is reserved for the most critical actions, such as "Launch" or "Add Instance."
- **Neutrals:** Uses a tiered system. `#12141C` is the base canvas. `#1B1E29` is used for primary containers and sidebars. `#20232F` is used for interactive hover states and nested cards.
- **Statuses:** High-saturation semantic colors provide immediate feedback on instance health and update status. 
- **Gradients:** Apply a linear gradient from `primary_color_hex` at 0% to `secondary_color_hex` at 100% for primary buttons and active selection glows.

## Typography

This design system uses **Inter** for its exceptional legibility at small sizes and its neutral, technical appearance.

- **Headlines:** Use Bold (700) or SemiBold (600) for major section titles and instance names.
- **Body Text:** Standard body text is set to 14px for balance between information density and readability.
- **Secondary Text:** Use the `text-secondary` color for descriptions and metadata to maintain hierarchy.
- **Labels:** Monospaced-style labels are preferred for version numbers and technical stats (e.g., "1.21.1") to emphasize the data-driven nature of the app.

## Layout & Spacing

The design system employs a **Fixed Sidebar + Fluid Content** model.

- **Grid:** Content is organized into a flexible grid of cards within the main viewport.
- **Density:** High density is preferred. Use a baseline 4px/8px spacing system for consistent alignment.
- **Sidebars:** The left sidebar is fixed at 240px. The right detail panel is optional and fixed at 320px; when active, it pushes the main content or overlays it on smaller window sizes.
- **Breakpoints:** 
    - **Desktop (1440px+):** Full 3-column layout (Sidebar, Grid, Detail Panel).
    - **Laptop (1024px - 1439px):** Detail panel becomes a slide-over drawer.
    - **Compact (960px):** Sidebar collapses to an icon-only "rail" (72px).

## Elevation & Depth

Visual hierarchy is established through **Tonal Layering** and **Subtle Blurs**.

- **Base Layer:** `#12141C` (background).
- **Mid Layer:** `#1B1E29` (sidebar, header, and primary cards). No shadow, defined by color contrast.
- **Top Layer:** `#20232F` (hover states, dropdowns, and modals).
- **Accents:** Active selections use a 1px solid border with the primary gradient and a subtle outer glow (`0 0 12px rgba(108, 92, 231, 0.3)`).
- **Glassmorphism:** Apply a 12px backdrop blur to modal backgrounds and the top navigation bar to create a sense of verticality.

## Shapes

The shape language is consistently **Rounded**, creating a friendly yet organized feel.

- **Primary Cards:** 12px border-radius.
- **Buttons & Inputs:** 8px border-radius for a more precise, tool-like appearance.
- **Status Pills:** Fully rounded (pill-shaped) for visual distinction from interactive buttons.
- **Instance Icons:** 12px radius to match the parent card container.

## Components

### Buttons
- **Primary:** Gradient background (#6C5CE7 -> #7C6FF0), white text, 8px radius. Pulsates during "Launch" loading states.
- **Secondary:** Surface-elevated background, white text. Hover state increases brightness.
- **Ghost:** No background, `text-secondary` color. Used for sidebar nav and utility links.

### Cards (Instances)
- A vertical stack with an icon (top/center), title (bold), and metadata (small, secondary). 
- **Selection State:** 1px primary gradient border and a corner "active" badge or checkbox.

### Input Fields & Search
- Dark background (#12141C), 8px radius.
- Includes a leading icon and a trailing "Hot-key" indicator (e.g., `Ctrl + K`).

### Navigation Sidebar
- Vertical items with 8px spacing.
- Active item: Background tint of primary color (10% opacity) and a 3px vertical "indicator" on the left edge.

### Right Detail Panel
- Highly structured. Section headers in `label-sm` (uppercase). 
- List items with leading icons for actions (Edit, Export, Delete).
- "Delete" action uses `status-error` for text and icon.

### Status Indicators
- Small circular dots or text pills using `status-success`, `status-warning`, or `status-error`. 
- High-contrast background with low-opacity fills for readability.