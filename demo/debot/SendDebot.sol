pragma ton-solidity >= 0.51.0;
pragma AbiHeader expire;
pragma AbiHeader time;
pragma AbiHeader pubkey;
import "https://raw.githubusercontent.com/tonlabs/debots/main/Debot.sol";
import "https://raw.githubusercontent.com/tonlabs/DeBot-IS-consortium/main/Terminal/Terminal.sol";
import "https://raw.githubusercontent.com/tonlabs/DeBot-IS-consortium/main/UserInfo/UserInfo.sol";

interface IMultisig {
    function sendTransaction(
        address dest,
        uint128 value,
        bool bounce,
        uint8 flags,
        TvmCell payload)
    external;
}

interface IOnSend {
    function onSend(bool succeed, uint32 sdkError, uint32 exitCode) external;
}

contract sendDebot is Debot {
    
    bytes m_icon;
    address m_invoker;
    address m_wallet;
    address m_dest;
    uint128 m_amount;
    bool m_bounce;
    uint32 m_sboxHandle;
    uint256 m_key;

    function setIcon(bytes icon) public {
        require(msg.pubkey() == tvm.pubkey(), 100);
        tvm.accept();
        m_icon = icon;
    }

    /// @notice Returns Metadata about DeBot.
    function getDebotInfo() public functionID(0xDEB) override view returns(
        string name, string version, string publisher, string caption, string author,
        address support, string hello, string language, string dabi, bytes icon
    ) {
        name = "sendDebot";
        version = "0.1.0";
        publisher = "Ever Surf";
        caption = "For testing";
        author = "Ever Surf";
        support = address(0);
        hello = "sendDebot";
        language = "en";
        dabi = m_debotAbi.get();
        icon = m_icon;
    }

    function getRequiredInterfaces() public view override returns (uint256[] interfaces) {
        return [Terminal.ID, UserInfo.ID];
    }

    /// @notice Entry point function for DeBot.
    function start() public override {
    }

    function send() public view {
        TvmCell empty;
        IMultisig(m_wallet).sendTransaction{
            time: 0, expire: 0, pubkey: m_key, 
            sign: true, signBoxHandle: m_sboxHandle,
            callbackId: tvm.functionId(onSuccess),
            onErrorId: tvm.functionId(onError)
        }(m_dest, m_amount, m_bounce, 3, empty).extMsg;
    }

    function onSuccess() public view { 
        IOnSend(m_invoker).onSend(true, 0, 0);
    }

    function onError(uint32 sdkError, uint32 exitCode) public view {
        IOnSend(m_invoker).onSend(false, sdkError, exitCode);
    }

    //
    // Invoke funcitons
    //

    function invokeSend(address dest, uint128 amount, bool bounce) public {
        m_invoker = msg.sender;
        m_dest = dest;
        m_amount = amount;
        m_bounce = bounce;
        UserInfo.getPublicKey(tvm.functionId(setKey));
        UserInfo.getAccount(tvm.functionId(setAccount));
        UserInfo.getSigningBox(tvm.functionId(setSBox));
    }

    // ----------------------------------------------------

    function setSBox(uint32 handle) public {
        require(handle != 0, 1001);
        m_sboxHandle = handle;
        Terminal.print(tvm.functionId(send), format("sbox handle = {}", m_sboxHandle));
    }

    function setAccount(address value) public {
        m_wallet = value;
    }

    function setKey(uint256 value) public {
        m_key = value;
    }
}