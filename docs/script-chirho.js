/* For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. (John 3:16) */

const revealElementsChirho = document.querySelectorAll("[data-reveal-chirho]");
const counterElementsChirho = document.querySelectorAll("[data-counter-chirho]");
const yearElementChirho = document.getElementById("year-chirho");

if (yearElementChirho) {
  yearElementChirho.textContent = String(new Date().getFullYear());
}

const prefersReducedMotionChirho = window.matchMedia(
  "(prefers-reduced-motion: reduce)"
).matches;

revealElementsChirho.forEach((elementChirho, indexChirho) => {
  const revealDelayChirho = prefersReducedMotionChirho ? 0 : indexChirho * 70;
  window.setTimeout(() => {
    elementChirho.classList.add("is-visible-chirho");
  }, revealDelayChirho);
});

const formatCounterValueChirho = (valueChirho) =>
  new Intl.NumberFormat("en-US").format(valueChirho);

const animateCounterChirho = (elementChirho) => {
  const targetTextChirho = elementChirho.dataset.counterChirho ?? "0";
  const targetValueChirho = Number.parseInt(targetTextChirho, 10);

  if (!Number.isFinite(targetValueChirho)) {
    return;
  }

  const durationChirho = 1300;
  const startedAtChirho = performance.now();

  const tickCounterChirho = (nowChirho) => {
    const elapsedChirho = nowChirho - startedAtChirho;
    const progressChirho = Math.min(elapsedChirho / durationChirho, 1);
    const easedProgressChirho = 1 - Math.pow(1 - progressChirho, 3);
    const currentValueChirho = Math.round(targetValueChirho * easedProgressChirho);
    elementChirho.textContent = formatCounterValueChirho(currentValueChirho);

    if (progressChirho < 1) {
      window.requestAnimationFrame(tickCounterChirho);
    }
  };

  window.requestAnimationFrame(tickCounterChirho);
};

const counterObserverChirho = new IntersectionObserver(
  (entriesChirho) => {
    entriesChirho.forEach((entryChirho) => {
      if (entryChirho.isIntersecting) {
        animateCounterChirho(entryChirho.target);
        counterObserverChirho.unobserve(entryChirho.target);
      }
    });
  },
  {
    threshold: 0.45,
  }
);

counterElementsChirho.forEach((counterElementChirho) => {
  counterObserverChirho.observe(counterElementChirho);
});
