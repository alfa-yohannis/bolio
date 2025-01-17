// animations.js

document.addEventListener('DOMContentLoaded', () => {
  const elements = document.querySelectorAll('.animate-on-scroll');

  const observer = new IntersectionObserver(entries => {
    entries.forEach(entry => {
      if (entry.isIntersecting) {
        entry.target.classList.add('opacity-100', 'translate-y-0');
      } else {
        entry.target.classList.remove('opacity-100', 'translate-y-0');
      }
    });
  }, {
    threshold: 0.1
  });

  elements.forEach(element => {
    element.classList.add('opacity-0', 'translate-y-10', 'transition-all', 'duration-700');
    observer.observe(element);
  });
});
