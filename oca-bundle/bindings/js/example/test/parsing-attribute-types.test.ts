import { expect } from 'chai'
import { create_nested_attr_type_from_js } from 'oca.js'

describe('Parsing attribute types', () => {
	it('should be parsed', () => {
		const numericTypeJs = create_nested_attr_type_from_js("Numeric");
        const dateTimeTypeJs = create_nested_attr_type_from_js("DateTime");
        const reference = "refs:EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs";
        const nestedAttrTypeJs = create_nested_attr_type_from_js(reference);
        const arrayTypeWithNumericJs = create_nested_attr_type_from_js(["Numeric"]);
        const arrayTypeWithRefJs = create_nested_attr_type_from_js(["refs:EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs"]);
        const arrayOfArrayTypeWithRefJs = create_nested_attr_type_from_js([["refs:EF5ERATRBBN_ewEo9buQbznirhBmvrSSC0O2GIR4Gbfs"]]);
	}),

	it("shouldn't be parsed", () => {
		expect( () => create_nested_attr_type_from_js("Wrong")).to.throw("Attribute type Wrong doesn't exist")
		const reference = "refs:not_said";
		expect( () => create_nested_attr_type_from_js(reference)).to.throw("Invalid said: Unknown code")
	})
})