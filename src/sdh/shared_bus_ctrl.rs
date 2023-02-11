#[doc = "Register `shared_bus_ctrl` reader"]
pub struct R(crate::R<SHARED_BUS_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHARED_BUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHARED_BUS_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHARED_BUS_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `shared_bus_ctrl` writer"]
pub struct W(crate::W<SHARED_BUS_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHARED_BUS_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SHARED_BUS_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHARED_BUS_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `num_clk_pins` reader - "]
pub type NUM_CLK_PINS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_3` reader - "]
pub type RESERVED_3_R = crate::BitReader<bool>;
#[doc = "Field `num_int_pins` reader - "]
pub type NUM_INT_PINS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_7_6` reader - "]
pub type RESERVED_7_6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bus_width_preset` reader - "]
pub type BUS_WIDTH_PRESET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `reserved_15` reader - "]
pub type RESERVED_15_R = crate::BitReader<bool>;
#[doc = "Field `clk_pin_sel` reader - "]
pub type CLK_PIN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `clk_pin_sel` writer - "]
pub type CLK_PIN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHARED_BUS_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_19` reader - "]
pub type RESERVED_19_R = crate::BitReader<bool>;
#[doc = "Field `int_pin_sel` reader - "]
pub type INT_PIN_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `int_pin_sel` writer - "]
pub type INT_PIN_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHARED_BUS_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `reserved_23` reader - "]
pub type RESERVED_23_R = crate::BitReader<bool>;
#[doc = "Field `bend_pwr_ctrl` reader - "]
pub type BEND_PWR_CTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `bend_pwr_ctrl` writer - "]
pub type BEND_PWR_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHARED_BUS_CTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `reserved_31` reader - "]
pub type RESERVED_31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn num_clk_pins(&self) -> NUM_CLK_PINS_R {
        NUM_CLK_PINS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn reserved_3(&self) -> RESERVED_3_R {
        RESERVED_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn num_int_pins(&self) -> NUM_INT_PINS_R {
        NUM_INT_PINS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn reserved_7_6(&self) -> RESERVED_7_6_R {
        RESERVED_7_6_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn bus_width_preset(&self) -> BUS_WIDTH_PRESET_R {
        BUS_WIDTH_PRESET_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn reserved_15(&self) -> RESERVED_15_R {
        RESERVED_15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn clk_pin_sel(&self) -> CLK_PIN_SEL_R {
        CLK_PIN_SEL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn reserved_19(&self) -> RESERVED_19_R {
        RESERVED_19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn int_pin_sel(&self) -> INT_PIN_SEL_R {
        INT_PIN_SEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn reserved_23(&self) -> RESERVED_23_R {
        RESERVED_23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn bend_pwr_ctrl(&self) -> BEND_PWR_CTRL_R {
        BEND_PWR_CTRL_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn reserved_31(&self) -> RESERVED_31_R {
        RESERVED_31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pin_sel(&mut self) -> CLK_PIN_SEL_W<16> {
        CLK_PIN_SEL_W::new(self)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    #[must_use]
    pub fn int_pin_sel(&mut self) -> INT_PIN_SEL_W<20> {
        INT_PIN_SEL_W::new(self)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    #[must_use]
    pub fn bend_pwr_ctrl(&mut self) -> BEND_PWR_CTRL_W<24> {
        BEND_PWR_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shared Bus Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shared_bus_ctrl](index.html) module"]
pub struct SHARED_BUS_CTRL_SPEC;
impl crate::RegisterSpec for SHARED_BUS_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shared_bus_ctrl::R](R) reader structure"]
impl crate::Readable for SHARED_BUS_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shared_bus_ctrl::W](W) writer structure"]
impl crate::Writable for SHARED_BUS_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets shared_bus_ctrl to value 0"]
impl crate::Resettable for SHARED_BUS_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
