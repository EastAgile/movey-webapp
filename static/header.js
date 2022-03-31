class Header {
  constructor(id) {
    this.id = id;
    this.display();
  }

  display() {
    const wrapper = document.getElementById(this.id);
    wrapper.innerHTML = `
    <header>
        <div class="header-container">
          <a class="sub-title" href="#">MOVEY</a>
          <nav>
            <ul>
              <li><a href="#">About</a></li>
              <li><a href="#">Documentation</a></li>
              <li><a href="#">Community</a></li>
            </ul>
            <ul>
              <li><button class="sign-in">SIGN IN</button></li>
              <li><button class="sign-up">SIGN UP</button></li>
            </ul>
          </nav>
        </div>
      </header>`;
  }
}
