import React, { Component } from "react";

import Wrapper from "./components/Wrapper";
import UIWrapper from "./components/UIWrapper";
import Canvas from "./components/Canvas";
import ToolBar from "./components/ToolBar";

class App extends Component {
  constructor() {
    super();

    this.state = {
      colors: [
        "white",
        "orange",
        "red",
        "yellow",
        "blue",
        "green",
        "darkBlue",
        "purple",
        "pink",
        "lavender",
        "teal",
        "lightBlue",
        "tan",
        "black",
        "lightGreen",
        "brown"
      ],
      imageAddress: "",
      currColor: "",
      image: "",
      width: 256,
      height: 256
    };

    this.handleClick = this.handleClick.bind(this);
    this.handleCanvasClick = this.handleCanvasClick.bind(this);
    this.renderButtons = this.renderButtons.bind(this);

    this.updateBackgroundImage();
  }

  renderButtons() {
    return this.state.colors.map(color => (
      <button
        onClick={() => this.handleClick(this.state.colors.indexOf(color))}
        style={{ backgroundColor: color }}
        color={color}
      >
        {color}
      </button>
    ));
  }

  handleClick(event, color) {
    this.setState({ currColor: arguments[0] });
  }

  handleCanvasClick(event) {
    console.log(event.nativeEvent.offsetX);

    const image = document.getElementById("canvas");
    const scaledX = Math.ceil(event.nativeEvent.offsetX / (image.offsetWidth / this.state.width));
    const scaledY = Math.ceil(event.nativeEvent.offsetY / (image.offsetHeight / this.state.height));

    this.sendEvent(
      scaledX,
      scaledY,
      this.state.currColor
    );
  }

  async sendEvent(x, y, color) {
    const url = `/write/${x}/${y}/${color}`;
    console.log(url);
    this.fetch(url);
    this.updateBackgroundImage();
  }

  async updateBackgroundImage() {
    this.setState({image: await this.fetchImage("http://localhost/board.bmp")});
  }

  async fetch(url) {
    try {
      return await window.fetch(url);
    } catch(e) {
      console.log(e);
    }
  }

  async fetchImage(url) {
    const dogImage = await this.fetch(url);
    console.log(dogImage);
    const dogBlob = await dogImage.blob();
    console.log(dogBlob);
    const urlCreator = window.URL || window.webkitURL;
    return urlCreator.createObjectURL(dogBlob);
  }

  render() {
    return (
      <Wrapper>
        <UIWrapper>
          <Canvas id="canvas" onClick={this.handleCanvasClick} src={this.state.image}/>
          <ToolBar>{this.renderButtons()}</ToolBar>
        </UIWrapper>
      </Wrapper>
    );
  }
}

export default App;
