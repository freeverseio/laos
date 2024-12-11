import chai, { expect } from "chai";
import { describeWithExistingNode } from "@utils/setups";
import { CHOPSTICKS_LAOS_NODE_IP } from "@utils/constants";

describeWithExistingNode(
	"Runtime upgrade",
	function () {},
	// Override LAOS node ip as this test is run with chopsticks
	CHOPSTICKS_LAOS_NODE_IP
);
