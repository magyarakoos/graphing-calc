import('./pkg')
  .catch(console.error);

import { render_canvas } from './pkg';

const input = document.querySelector('#input');
let timeout;
input.addEventListener('input', () => {
    render_canvas(input.value)
})
