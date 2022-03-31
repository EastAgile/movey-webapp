class Footer {
  constructor(id) {
    this.id = id;
    this.display();
  }

  display() {
    const wrapper = document.getElementById(this.id);
    wrapper.innerHTML = `
    <footer>
      <div class="footer-container">
        <div class="footer-about">
          <div class="title">MOVEY</div>
          <div class="about-us">
            We're East Agile, the company behind Movey, the Move package manager, the Movey Registry.
          </div>
        </div>
        <div class="footer-center">
          <div class="link-container">
            <a class="footer-link" href="#">OUR TEAM</a>
            <span>/</span>
            <a class="footer-link" href="#">CONTACT US</a>
            <span>/</span>
            <a class="footer-link" href="#">TERMS & CONDITIONS</a>
            <span>/</span>
            <a class="footer-link" href="#">SECURITY POLICY</a>
          </div>
          <ul class="social-icons">
            <li>
              <a class="icon fab fa-twitter" href="#" target="_blank"></a>
            </li>
            <li>
              <a class="icon fab fa-medium" href="#" target="_blank"></a>
            </li>
            <li>
              <a class="icon fab fa-github" href="#" target="_blank"></a>
            </li>
            <li>
              <a class="icon fab fa-reddit" href="#" target="_blank"></a>
            </li>
          </ul>
        </div>
        <div class="footer-copyright">
          <div class="copyright">
          © 2021 Tokners. All rights reserved.
          </div>
        </div>
    </footer>`;
  }
}
