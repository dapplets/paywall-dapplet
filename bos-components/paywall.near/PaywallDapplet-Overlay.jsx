/**
 * We use buyerAccountId instead of accountId because it's reserved parameter in embeded pages
 * https://github.com/near/near-discovery/blob/dc58aa1c8ef5d4c0a5e19839230148009834088a/src/pages/embed/%5BaccountId%5D/widget/%5BcomponentName%5D.tsx#L16
 */
const { buyerAccountId } = props;

if (!buyerAccountId) {
  return <div>Connect your Wallet</div>;
}

const purchases = buyerAccountId
  ? Near.view(
      "app.paywall.near",
      "purchases",
      {
        account_id: buyerAccountId,
      },
      "final",
      true
    )
  : false;

const Wrapper = styled.div`
  .main-container {
    padding: 10px;
  }

  .content-card {
    margin-bottom: 20px;
    width: 100%;
  }
`;

if (purchases === null) {
  return <div>Loading...</div>;
}

return (
  <Wrapper>
    <div className="main-container">
    {purchases.map((contentId) => (
      <div className="content-card">
        <div>Tweet ID: {contentId}</div>
        <Widget
          key={contentId}
          src={`paywall.near/widget/PaywallDapplet-Content`}
          props={{ accountId: buyerAccountId, contentId: contentId }}
        />
      </div>
    ))}
    </div>
  </Wrapper>
);
