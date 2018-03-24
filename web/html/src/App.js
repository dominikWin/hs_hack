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
        { color: "red", hex: "#B03A2E" },
        { color: "orange", hex: "#E67E22" },
        { color: "yellow", hex: "#F4DO3F" },
        { color: "green", hex: "#52BE80" },
        { color: "blue", hex: "#3498DB" },
        { color: "darkBlue", hex: "#1F618" },
        { color: "purple", hex: "#7D3C9F" },
        { color: "pink", hex: "#E91E63" },
        { color: "white", hex: "#FDFEFE" },
        { color: "lavender", hex: "#C39BD3" },
        { color: "teal", hex: "#76D7C4" },
        { color: "lightBlue", hex: "#D6EAFF" },
        { color: "tan", hex: "#EDBB99" },
        { color: "brown", hex: "#A04000" },
        { color: "black", hex: "#17202A" },
        { color: "lightGreen", hex: "#81C784" }
      ],
      imageAddress: "",
      currColor: "",
      image: ""
    };

    this.handleClick = this.handleClick.bind(this);
    this.handleCanvasClick = this.handleCanvasClick.bind(this);
    this.renderButtons = this.renderButtons.bind(this);
  }

  renderButtons() {
    return this.state.colors.map(color => (
      <button
        onClick={() => this.handleClick(color.color)}
        style={{ backgroundColor: color.color }}
        color={color.color}
      >
        {color.color}
      </button>
    ));
  }

  handleClick(event, color) {
    this.setState({ currColor: arguments[0] });
  }

  handleCanvasClick(event) {
    console.log(event.nativeEvent.offsetX);
    this.sendEvent(
      event.nativeEvent.offsetX,
      event.nativeEvent.offsetY,
      this.state.currColor
    );
  }

  async sendEvent(x, y, color) {
    const url = `/api/${x}/${y}/${color}`;
    console.log(url);
    try {
      const dogImage = await window.fetch("http://localhost:3000/cutting-board.jpg");
      console.log(dogImage);
      const dogBlob = await dogImage.blob();
      console.log(dogBlob);
      const urlCreator = window.URL || window.webkitURL;
      const dogAddress = urlCreator.createObjectURL(dogBlob);
      this.setState({image: dogAddress});
    } catch(e) {
      console.log(e);
    }
  }

  updateImage() {
    this.setstate({ imageAddress: `/api/` });
  }

  render() {
    return (
      <Wrapper>
        <UIWrapper>
          <Canvas onClick={this.handleCanvasClick} src={this.state.image}/>
          <ToolBar>{this.renderButtons()}</ToolBar>
        </UIWrapper>
      </Wrapper>
    );
  }
}

export default App;
