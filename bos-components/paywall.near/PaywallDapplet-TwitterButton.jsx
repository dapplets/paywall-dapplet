const Wrapper = styled.div`
  .button-default {
    cursor: pointer;
    user-select: none;

    box-sizing: border-box;
    height: 35px;
    margin: 0 8px 12px 0;
    padding: 0 10px;

    font-family: TwitterChirp, -apple-system, BlinkMacSystemFont, "Segoe UI",
      Roboto, Helvetica, Arial, sans-serif;
    font-size: 15px;
    font-weight: 600;
    line-height: 33px;

    background: inherit;
    border-radius: 9999px;

    transition: background-color 0.2s ease-in-out;
    display: flex;
  }

  .button-light {
    color: #000;
    border: 1px solid rgb(207 217 222);
  }

  .button-light:hover {
    background-color: rgb(15 20 25 / 10%);
  }

  .button-dark {
    color: #fff;
    border: 1px solid rgb(83 100 113);
  }

  .button-dark:hover {
    background-color: rgb(239 243 244 / 10%);
  }

  .button-img-display {
    fill: currentcolor;
    height: auto;
    max-width: 100%;
    position: relative;
    vertical-align: text-bottom;
    width: 1.25em;
    margin: auto 0;
    display: flex !important;
    align-items: center !important;
  }
`;

return (
  <Wrapper>
    <div
      role="button"
      className={`button-default ${
        props.dark ? "button-dark" : "button-light"
      }`}
      onClick={() => props.onClick?.()}
      style={{
        opacity: props.disabled ? ".5" : "1",
      }}
      disabled={props.disabled}
    >
      <div
        style={{
          opacity: props.disabled ? ".5" : "1",
          display: "flex",
          alignItems: "center",
          marginRight: props.label?.toString() ? "12px" : 0,
        }}
      >
        <img src={props.icon} className="button-img-display" />
      </div>
      <div>
        <span>{props.label}</span>
      </div>
    </div>
  </Wrapper>
);
