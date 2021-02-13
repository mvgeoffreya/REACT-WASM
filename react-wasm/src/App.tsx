import React, { useEffect } from 'react';
import logo from './logo.svg';
import './App.css';
const basic_html = import('wasm')

const titleStyle = `
    style="
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 90vh;
    overflow: hidden;
    "
`
const author = `<p> Geoffrey Aaron </p>`;



function App() {
  useEffect(() => {
    const ref = document.getElementsByClassName('App')[0];
    basic_html.then(data => {
      console.log('test')
      data.generate(
        `<div $${titleStyle}> 
          <div> 
              <h1> This Website is powered by WASM </h1>
          </div>
          <div> 
              ${author}
          </div> 
      </div>`, ref
      )
    })
  });

  return (
    <div className="App">
    </div>
  );
}

export default App;
