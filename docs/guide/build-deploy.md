# Build & Deployment

Guide for building and deploying HTMS applications to production.

## Build Process

### Development Build

```bash
# Watch mode for development
htms compile src/app.htms -o src/dist/ --watch

# Or with Vite
npm run dev
```

### Production Build

```bash
# Compile HTMS
htms compile src/app.htms -o src/dist/

# Build with Vite
npm run build
```

---

## Output Formats

### TypeScript Output (For Dynamic Apps)

**Use when:**
- Building SPAs (Single Page Applications)
- Need dynamic data from APIs
- Client-side routing with state management

**Build:**
```bash
htms compile app.htms -o dist/
```

**Generated files:**
- `dist/templates.ts` - Component functions
- `dist/router.ts` - Router and context management
- `dist/events.ts` - Event system

**Then build with bundler:**
```bash
# Vite
npm run build

# Or Webpack
npm run webpack:build
```

### HTML Output (For Static Sites)

**Use when:**
- Building marketing sites
- Landing pages
- Blogs
- Documentation sites
- Content-focused websites

**Build:**
```bash
htms compile app.htms -o dist/ --format html
```

**Generated:**
- `dist/index.html` - Complete standalone HTML file

**Deploy directly** to:
- Netlify
- Vercel
- GitHub Pages
- S3 + CloudFront
- Any static hosting

---

## Deployment Platforms

### Netlify

**1. Build settings:**
```toml
# netlify.toml
[build]
  command = "htms compile src/app.htms -o dist/ --format html"
  publish = "dist"
```

**2. Deploy:**
```bash
netlify deploy --prod
```

**Or via Git:**
1. Push to GitHub
2. Connect repo in Netlify dashboard
3. Set build command and publish directory
4. Deploy!

### Vercel

**1. Create `vercel.json`:**
```json
{
  "buildCommand": "htms compile src/app.htms -o dist/ --format html",
  "outputDirectory": "dist"
}
```

**2. Deploy:**
```bash
vercel --prod
```

### GitHub Pages

**1. Build:**
```bash
htms compile src/app.htms -o dist/ --format html
```

**2. Deploy:**
```bash
# Copy to gh-pages branch
git add dist
git commit -m "Build"
git subtree push --prefix dist origin gh-pages
```

**Or use GitHub Actions:**
```yaml
# .github/workflows/deploy.yml
name: Deploy to GitHub Pages

on:
  push:
    branches: [ main ]

jobs:
  build-deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install HTMS
        run: npm install -g @progalaxyelabs/htms-cli

      - name: Build
        run: htms compile src/app.htms -o dist/ --format html

      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```

### AWS S3 + CloudFront

**1. Build:**
```bash
htms compile src/app.htms -o dist/ --format html
```

**2. Upload to S3:**
```bash
aws s3 sync dist/ s3://your-bucket-name --delete
```

**3. Invalidate CloudFront:**
```bash
aws cloudfront create-invalidation \
  --distribution-id YOUR_DIST_ID \
  --paths "/*"
```

**Or use script:**
```bash
#!/bin/bash
htms compile src/app.htms -o dist/ --format html
aws s3 sync dist/ s3://your-bucket --delete
aws cloudfront create-invalidation --distribution-id $DIST_ID --paths "/*"
```

### DigitalOcean App Platform

**1. Create `app.yaml`:**
```yaml
name: htms-app
static_sites:
  - name: web
    build_command: htms compile src/app.htms -o dist/ --format html
    output_dir: dist
```

**2. Deploy:**
```bash
doctl apps create --spec app.yaml
```

---

## Build Optimization

### Minification

**For HTML output:**

The generated HTML includes inline JavaScript. Minify it:

```bash
# Install minifier
npm install -g html-minifier

# Minify
html-minifier --collapse-whitespace \
  --remove-comments \
  --minify-js true \
  --minify-css true \
  dist/index.html \
  -o dist/index.min.html
```

**For TypeScript output:**

Your bundler (Vite, Webpack) handles this:

```typescript
// vite.config.ts
import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true
      }
    }
  }
});
```

### Code Splitting

**HTML mode with split templates:**

```bash
htms compile app.htms -o dist/ \
  --format html \
  --split-templates
```

This generates:
- `dist/index.html` - Main file
- `dist/templates/home.html` - Home page template
- `dist/templates/about.html` - About page template

Templates are loaded on-demand.

**TypeScript mode:**

Vite/Webpack handle code splitting:

```typescript
// Dynamic imports in your code
const module = await import('./heavy-module');
```

### Asset Optimization

**Images:**

```bash
# Install optimizer
npm install -g imagemin-cli

# Optimize
imagemin src/images/* --out-dir=dist/images
```

**CSS:**

```bash
# Install PurgeCSS
npm install -D @fullhuman/postcss-purgecss

# Configure in postcss.config.js
module.exports = {
  plugins: [
    require('@fullhuman/postcss-purgecss')({
      content: ['./dist/**/*.html', './dist/**/*.js'],
      defaultExtractor: content => content.match(/[\w-/:]+(?<!:)/g) || []
    })
  ]
}
```

---

## Environment Variables

### TypeScript Output

Use Vite's env variables:

**`.env.production`:**
```
VITE_API_URL=https://api.example.com
VITE_ANALYTICS_ID=GA-XXXXX
```

**Access in code:**
```typescript
const apiUrl = import.meta.env.VITE_API_URL;
```

### HTML Output

Replace at build time:

```bash
# Build
htms compile app.htms -o dist/ --format html

# Replace variables
sed -i 's|__API_URL__|https://api.example.com|g' dist/index.html
```

Or use a template:

```bash
htms compile app.htms -o dist/ \
  --format html \
  --template template.html
```

**template.html:**
```html
<!DOCTYPE html>
<html>
<head>
  <script>
    window.ENV = {
      API_URL: '__API_URL__',
      ANALYTICS_ID: '__ANALYTICS_ID__'
    };
  </script>
</head>
<body>
  <!-- HTMS templates injected here -->
</body>
</html>
```

---

## CI/CD Pipeline

### GitHub Actions Example

```yaml
# .github/workflows/ci-cd.yml
name: CI/CD

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build-and-deploy:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout
        uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: '18'

      - name: Install dependencies
        run: npm ci

      - name: Install HTMS CLI
        run: npm install -g @progalaxyelabs/htms-cli

      - name: Lint HTMS
        run: htms check src/app.htms

      - name: Compile HTMS
        run: htms compile src/app.htms -o src/dist/

      - name: Build
        run: npm run build

      - name: Test
        run: npm test

      - name: Deploy to Netlify
        if: github.ref == 'refs/heads/main'
        run: |
          npm install -g netlify-cli
          netlify deploy --prod --dir=dist
        env:
          NETLIFY_AUTH_TOKEN: ${{ secrets.NETLIFY_AUTH_TOKEN }}
          NETLIFY_SITE_ID: ${{ secrets.NETLIFY_SITE_ID }}
```

---

## Performance Optimization

### Lazy Loading

**Images:**
```htms
img [src: item.url, loading: "lazy", alt: item.title]
```

**Routes (TypeScript output):**

Modify generated router.ts to lazy load pages:

```typescript
const routes = {
  '/': () => import('./pages/home').then(m => renderPage(m.HomePage)),
  '/about': () => import('./pages/about').then(m => renderPage(m.AboutPage))
};
```

### Caching

**Service Worker (for PWA):**

```javascript
// sw.js
const CACHE_NAME = 'htms-app-v1';
const urlsToCache = [
  '/',
  '/dist/templates.js',
  '/dist/router.js',
  '/styles.css'
];

self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then(cache => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', event => {
  event.respondWith(
    caches.match(event.request)
      .then(response => response || fetch(event.request))
  );
});
```

**HTTP Headers:**

Configure your server/CDN:

```
# Netlify _headers
/dist/*
  Cache-Control: public, max-age=31536000, immutable

/*.html
  Cache-Control: public, max-age=0, must-revalidate

/images/*
  Cache-Control: public, max-age=31536000
```

### CDN

Use a CDN for static assets:

```html
<link rel="stylesheet" href="https://cdn.example.com/styles.css">
<script src="https://cdn.example.com/dist/templates.js"></script>
```

---

## Monitoring

### Error Tracking

**Sentry:**

```typescript
// main.ts
import * as Sentry from "@sentry/browser";

Sentry.init({
  dsn: "YOUR_SENTRY_DSN",
  environment: "production"
});

// Catch errors in actions
export const actions = {
  handleAction: (ctx, event) => {
    try {
      // Your logic
    } catch (error) {
      Sentry.captureException(error);
      throw error;
    }
  }
};
```

### Analytics

**Google Analytics:**

```html
<!-- In index.html -->
<script async src="https://www.googletagmanager.com/gtag/js?id=GA-XXXXXX"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());
  gtag('config', 'GA-XXXXXX');
</script>
```

**Track page views:**

```typescript
// In router.ts after rendering
gtag('event', 'page_view', {
  page_path: window.location.hash
});
```

---

## Security

### Content Security Policy

```html
<meta http-equiv="Content-Security-Policy"
  content="default-src 'self';
           script-src 'self' 'unsafe-inline';
           style-src 'self' 'unsafe-inline';
           img-src 'self' data: https:;">
```

### HTTPS

Always deploy with HTTPS. Most platforms (Netlify, Vercel) provide this automatically.

### Input Sanitization

HTMS uses pure DOM API (no innerHTML), making it XSS-safe by default. However, always validate user input:

```typescript
export const actions = {
  submitComment: (ctx, event) => {
    const comment = ctx.data.newComment.trim();

    // Validate
    if (comment.length === 0) return;
    if (comment.length > 500) {
      ctx.data.error = 'Comment too long';
      ctx.rerender();
      return;
    }

    // Submit...
  }
};
```

---

## Troubleshooting

### Build fails with "Component not found"

→ Ensure components are defined before pages that use them

### Assets not loading in production

→ Check asset paths are relative or use absolute URLs

### Router not working after deployment

→ Configure server for hash-based routing (usually works out of the box)

For history mode (no hash), configure server to serve index.html for all routes

### Large bundle size

→ Enable code splitting and tree shaking
→ Use `--split-templates` for HTML output
→ Lazy load heavy dependencies

---

## Checklist

Before deploying to production:

- [ ] Run `htms check` to validate syntax
- [ ] Test all routes and interactions
- [ ] Optimize images and assets
- [ ] Enable minification
- [ ] Configure caching headers
- [ ] Set up error tracking (Sentry, etc.)
- [ ] Add analytics (GA, Plausible, etc.)
- [ ] Test on mobile devices
- [ ] Check loading performance (Lighthouse)
- [ ] Set up CI/CD pipeline
- [ ] Configure custom domain and HTTPS
- [ ] Add meta tags for SEO

---

## Next Steps

- [Component Patterns](/guide/component-patterns) - Best practices
- [Examples](/examples/) - Real-world applications
